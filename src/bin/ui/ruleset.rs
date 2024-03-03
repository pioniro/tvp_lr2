use ratatui::layout::Constraint::Length;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Cell, Row, Table, Widget};
use lr2::{Ruleset, RuleState};

pub(crate) struct RulesetWidget<'a> {
    ruleset: &'a Ruleset,
    state: RuleState,
    symbol: char,
}

impl<'a> RulesetWidget<'a> {
    pub(crate) fn new(ruleset: &'a Ruleset, rule: RuleState, symbol: char) -> Self {
        RulesetWidget { ruleset, state: rule, symbol }
    }
}

impl Widget for RulesetWidget<'_> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let rows = vec![Row::new(
            vec![Cell::from("".to_string())]
                .into_iter()
                .chain(self.ruleset.states().iter().map(|state|
                    match (Cell::from(state.to_string()), *state == self.state) {
                        (cell, true) => cell.on_dark_gray(),
                        (cell, false) => cell,
                    }))
                .collect::<Vec<_>>()
        ).on_gray()]
            .into_iter()
            .chain(
                self.ruleset
                    .alphabet()
                    .into_iter()
                    .map(|symbol| {
                        vec![match (Cell::from(symbol.to_string()).light_cyan(), *symbol == self.symbol) {
                            (cell, true) => cell.on_dark_gray(),
                            (cell, false) => cell,
                        }]
                            .into_iter()
                            .chain(self.ruleset
                                .states()
                                .iter()
                                .map(|state|
                                match (Cell::from(self.ruleset.find(state, symbol).unwrap().to_string()), *state == self.state, *symbol == self.symbol) {
                                    (cell, false, true) | (cell, true, false) => cell.on_dark_gray(),
                                    (cell, true, true)=> cell.on_blue(),
                                    (cell, false, false) => cell,
                                })
                                .collect::<Vec<Cell>>()
                            )
                    }).map(|row| Row::new(row).on_gray())
            );
        let cols_count = self.ruleset.states().len() + 1;
        Table::new(rows, vec![Length(5); cols_count]).render(area, buf);
    }
}