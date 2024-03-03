mod app;
mod history;
mod window;
mod tape;
mod ruleset;

use std::fs;
use std::fs::File;
use crossterm::{
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::prelude::{CrosstermBackend, Terminal};
use std::io::{stdout, Result, Error, ErrorKind, Write};
use std::path::Path;
use app::App;
use lr2::{Ruleset, Tape, Transition, Turing};
use std::str::FromStr;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    tape: String,
    #[arg(short, long)]
    rules: String,
    #[arg(short, long)]
    out: Option<String>,
    #[arg(long = "no-interactive", default_value = "false")]
    no_interactive: bool,
}

fn string_to_tape(s: String) -> Result<Tape> {
    let mut lines = s.lines();
    let tape_str = lines.next().ok_or_else(|| Error::new(ErrorKind::Other, "Tape doesnt found"))?;
    let start: isize = lines.next().ok_or_else(|| Error::new(ErrorKind::Other, "Start position doesnt found"))?.parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    Ok(Tape::new(tape_str.chars().collect(), start, 0))
}

fn main() -> Result<()> {
    let args = Args::parse();
    let tape_str = fs::read_to_string(args.tape.clone())?;
    let rules_str = fs::read_to_string(args.rules.clone())?;
    let out = open_output(args.out)?;

    let tape = string_to_tape(tape_str)?;
    let rules = Ruleset::from_str(rules_str.as_str()).map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    let mt = Turing::new(tape, 0, rules);
    if args.no_interactive {
        non_interactive(mt, out)
    } else {
        interactive(mt, out)
    }
}


fn interactive(turing: Turing, mut out: Box<dyn Write>) -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new(turing);
    app.run_ui(terminal)?;
    restore_terminal()?;
    write_history(app, out.as_mut())?;
    Ok(())
}

fn non_interactive(turing: Turing, mut out: Box<dyn Write>) -> Result<()> {
    let mut app = App::new(turing);
    app.history.add_listener(move |t, i| {
        write_transition(t, i, out.as_mut()).unwrap();
    });
    app.run()?;
    Ok(())
}

fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn open_output(out: Option<String>) -> Result<Box<dyn Write>> {
    Ok(match out {
        Some(x) => {
            let path = Path::new(&x);
            Box::new(File::create(&path)?) as Box<dyn Write>
        }
        None => Box::new(stdout()) as Box<dyn Write>,
    })
}

fn write_history(app: App, file: &mut dyn Write) -> Result<()> {
    app.history().iter().enumerate().try_for_each(|(i, t)| write_transition(t, i, file))?;
    Ok(())
}

fn write_transition(transition: &Transition, step: usize, file: &mut dyn Write) -> Result<()> {
    file.write_all(format!(
        "\
=============== Step: {} ===============
Tape:\t{}
State:\t\t{}\tReplace:\t{}
Next state:\t{}\tMove:\t\t{}
",
        step,
        transition.tape().to_string(),
        transition.state().to_string(),
        transition.rule().write(),
        transition.rule().next_state(),
        transition.rule().mov(),
    ).as_ref())?;
    Ok(())
}