use std::io::{Error, ErrorKind};
use std::time::Duration;
use std::time::Instant;
use crossterm::event;
use crossterm::event::{Event, KeyEventKind};
use crossterm::event::KeyCode;
use ratatui::backend::Backend;
use ratatui::Terminal;
use lr2::{Transition, Turing, TuringError};
use crate::window::Window;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub (crate) enum AppState {
    #[default]
    Running,
    Quit,
}
pub (crate) struct App {
    state: AppState,
    turing: Turing,
    frame_timeout: Duration,
    step_last: Instant,
    speed: u8,
    pub(crate) history: History,
    max_iteration: usize,
}

pub(crate) struct History {
    storage: Vec<Transition>,
    offset: usize,
    follow: bool,
    listeners: Vec<Box<dyn FnMut(&Transition, usize)>>,
}

impl History {
    pub(crate) fn new(storage: Vec<Transition>, offset: usize, follow: bool) -> History {
        History {
            storage,
            offset,
            follow,
            listeners: vec![],
        }
    }
    pub(crate) fn add_listener(&mut self, listener: impl FnMut(&Transition, usize) + 'static) {
        self.listeners.push(Box::new(listener));
    }

    pub(crate) fn add(&mut self, transition: Transition) {
        self.storage.push(transition.clone());
        self.notify(&transition);
    }
    pub(crate) fn notify(&mut self, x: &Transition) {
        for listener in &mut self.listeners {
            listener(x, self.storage.len());
        }
    }
}


impl App {
    pub(crate) fn new(turing: Turing) -> App {
        App {
            history: History::new(vec![], 0, true),
            state: AppState::Running,
            turing,
            frame_timeout: Duration::from_millis(250),
            step_last: Instant::now(),
            speed: 4,
            max_iteration: 1_000,
        }
    }
    pub (crate) fn run(&mut self) -> std::io::Result<()> {
        while self.is_running() {
            self.next_step().map_err(|e| Error::new(ErrorKind::Other, e))?;
        }
        Ok(())
    }
    pub (crate) fn run_ui(&mut self, mut terminal: Terminal<impl Backend>) -> std::io::Result<()> {
        while self.is_running() {
            self.update().map_err(|e| Error::new(ErrorKind::Other, e))?;
            self.handle_events()?;
            self.draw(&mut terminal)?;
        }
        Ok(())
    }

    fn update(&mut self) -> Result<(), TuringError> {
        if self.step_last.elapsed() > self.frame_timeout * self.speed as u32{
            self.step_last = Instant::now();
            return self.next_step();
        }
        Ok(())
    }

    fn next_step(&mut self) -> Result<(), TuringError> {
        self.turing.next_transition().map(|transition| {
            self.turing.apply_transition(&transition);
            if self.history.storage.len() >= self.max_iteration || transition.rule().mov().is_terminal() {
                self.state = AppState::Quit;
            }
            self.history.add(transition);
        }).map_err(|e| {
            self.state = AppState::Quit;
            e
        })
    }
    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> std::io::Result<()> {
        let window = Window::new(
            &self.history.storage,
            self.history.offset,
            self.history.follow,
            self.turing.tape(),
            self.turing.ruleset(),
            self.turing.state(),
            self.turing.tape().read()
        );
        terminal.draw(|frame| frame.render_widget(window, frame.size()))?;
        Ok(())
    }
    fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        use KeyCode::*;
        if event::poll(self.frame_timeout)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    Char('q') | Esc => self.quit(),
                    Down => self.scroll_down(),
                    Up => self.scroll_up(),
                    _ => (),
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn scroll_up(&mut self) {
        if self.history.follow {
            self.history.follow = false;
            self.history.offset = self.history.storage.len().saturating_sub(2);
        } else {
            self.history.offset = self.history.offset.saturating_sub(1);
        }
    }

    fn scroll_down(&mut self) {
        let history_scroll_offset = self.history.offset.saturating_add(1).min(self.history.storage.len()-1);
        if !self.history.follow && self.history.offset == history_scroll_offset {
            self.history.follow = true;
        }
        self.history.offset = history_scroll_offset;
    }
    fn quit(&mut self) {
        self.state = AppState::Quit;
    }
    pub(crate) fn history(&self) -> &Vec<Transition> {
        &self.history.storage
    }
}