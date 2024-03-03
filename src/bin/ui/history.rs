use std::cmp::min;
use ratatui::buffer::Buffer;
use ratatui::layout::{Layout, Rect};
use ratatui::layout::Constraint::{Length, Min};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Widget, Paragraph, Borders, ScrollbarState, Scrollbar, ScrollbarOrientation};
use lr2::{Transition};
use crate::tape::TapeWidget;

pub(crate) struct History<'a> {
    history: &'a Vec<Transition>,
    height: u16,
    follow: bool,
    scroll_offset: usize,
}

impl<'a> History<'a> {
    pub(crate) fn new(history: &'a Vec<Transition>, scroll_offset: usize, scroll_follow: bool) -> Self {
        History {
            history,
            height: 5,
            follow: scroll_follow,
            scroll_offset,
        }
    }
}

impl Widget for History<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let viewport_capacity = area.height as usize / self.height as usize;
        let scroll_offset = if self.follow {
            self.history.len().saturating_sub(viewport_capacity)
        } else {
            self.scroll_offset
        };
        let records_on_the_screen = min(viewport_capacity, self.history.len() - scroll_offset);
        let mut scroll_state = ScrollbarState::default()
            .content_length(self.history.len())
            .position(scroll_offset)
            .viewport_content_length(records_on_the_screen);
        Layout::vertical(vec![Length(self.height); records_on_the_screen])
            .split(area)
            .iter()
            .zip(self.history.iter().skip(scroll_offset))
            .enumerate()
            .for_each(|(i, (a, trans))| {
                let block = Block::default()
                    .title(format!("Step {}", i + scroll_offset + 1).italic())
                    .borders(Borders::ALL);
                let inner = block.inner(*a);
                let [rect_tape, rect_rule] = Layout::horizontal([Min(15), Length(5)]).areas(inner);
                TapeWidget::new(trans.tape()).render(rect_tape, buf);
                Paragraph::new(trans.rule().to_string()).render(rect_rule, buf);
                block.render(*a, buf);
            });
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .render(area, buf, &mut scroll_state)
        ;
    }
}