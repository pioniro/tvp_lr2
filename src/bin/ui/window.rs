use ratatui::buffer::Buffer;
use ratatui::layout::{Layout, Rect};
use ratatui::layout::Constraint::{Fill, Length, Min};
use ratatui::prelude::Widget;
use crate::history::History;
use ratatui::widgets::Block;
use lr2::{Ruleset, RuleState, Tape, Transition};
use crate::ruleset::RulesetWidget;
use crate::tape::TapeWidget;

pub (crate) struct Window<'a> {
    tape: TapeWidget<'a>,
    history: History<'a>,
    ruleset: RulesetWidget<'a>,
}
impl<'a> Window<'a> {
    pub (crate) fn new(
        history: &'a Vec<Transition>,
        scroll_offset: usize,
        scroll_follow: bool,
        tape: &'a Tape,
        ruleset: &'a Ruleset,
        state: RuleState,
        symbol: char,
    ) -> Self {
        Window {
            tape: TapeWidget::new(tape),
            history: History::new(history, scroll_offset, scroll_follow),
            ruleset: RulesetWidget::new(ruleset, state, symbol),
        }
    }
}

impl Widget for Window<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::horizontal([Min(80), Length(80)]);
        let [left, right] = layout.areas(area);
        let [tape_rect, ruleset_rect] = Layout::vertical([Length(4), Fill(1)]).areas(left);

        let tape_block = Block::default().title("Tape").borders(ratatui::widgets::Borders::ALL);
        self.tape.render(tape_block.inner(tape_rect), buf);
        tape_block.render(tape_rect, buf);
        let ruleset_block = Block::default().title("Rules").borders(ratatui::widgets::Borders::ALL);
        self.ruleset.render(ruleset_block.inner(ruleset_rect), buf);
        ruleset_block.render(ruleset_rect, buf);
        let history_block = Block::default().title("History").borders(ratatui::widgets::Borders::ALL);
        self.history.render(history_block.inner(right), buf);
        history_block.render(right, buf);
    }
}