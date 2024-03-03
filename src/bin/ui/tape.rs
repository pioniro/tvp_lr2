use ratatui::layout::Constraint::Length;
use ratatui::widgets::{Cell, Row, Widget};
use lr2::Tape;
use ratatui::prelude::Stylize;

pub (crate) struct TapeWidget<'a> {
    tape: &'a Tape,
}

impl<'a> TapeWidget<'a> {
    pub (crate) fn new(tape: &'a Tape) -> Self {
        TapeWidget { tape}
    }
}

impl Widget for TapeWidget<'_> {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        const WIDTH: u16 = 3;
        const MIN_SYMBOLS: usize = 3;
        let symbols = ((area.width / WIDTH) as usize).max(MIN_SYMBOLS);
        let index = self.tape.index();
        let vec = self.tape.data();
        let min = index.saturating_sub(symbols);
        let max = index.saturating_add(symbols).min(vec.len());
        let show_data = &vec[min..max];
        let local_index = index - min;

        let rows = [
            Row::new(show_data
                         .iter()
                         .enumerate()
                         .map(|(i ,_)| Cell::from(format!("{}", i + min)))
                         .enumerate()
                         .map(|(i, c)| if i == local_index { c.on_cyan() } else { c })
                         .collect::<Vec<Cell>>()
            ),
            Row::new(show_data
                         .iter()
                         .enumerate()
                         .map(|(_ ,c)| Cell::from(c.to_string()))
                         .enumerate()
                         .map(|(i, c)| if i == local_index { c.on_cyan() } else { c })
                .collect::<Vec<Cell>>()
            ),
        ];
        ratatui::widgets::Table::new(
            rows,
            vec![Length(WIDTH); show_data.len()],
        ).render(area, buf);
    }
}