//
//
//

use ratatui::{
    prelude::*,
    widgets::*,
};

pub struct Ui;

//
//
//
impl Widget for Ui {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::new().style(Style::new()
            .bg(Color::Black))
            .render(area, buf);
    }
}

//
//
//
impl Ui {
    pub fn new() -> Self {
        Self
    }
}
