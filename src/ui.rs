//
//
//

use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::app::App;

#[derive(Debug)]
pub struct Ui<'a, 'b> {
    app: &'a mut App<'b>,
}

//
//
//
impl Widget for Ui<'_, '_> {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header, content, status] = chunks.areas(area);

        Block::new().style(Style::new()
            .bg(Color::Black))
            .render(area, buf);

        self.render_title_bar(header, buf);
        self.render_content(content, buf);
        self.render_status_bar(status, buf);
    }
}

//
//
//
impl<'a, 'b> Ui<'a, 'b> {
    pub fn new(app: &'a mut App<'b>) -> Self {
        Self { app }
    }

    fn render_title_bar(&mut self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Min(0),
        ]);
        let [title, version] = chunks.areas(area);

        Paragraph::new(self.app.name.to_ascii_uppercase())
            .block(Block::new()
                .padding(Padding::horizontal(1)))
            .style(Style::new()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD))
            .render(title, buf);

        Paragraph::new(self.app.version().as_string())
            .alignment(Alignment::Right)
            .block(Block::new()
                .padding(Padding::horizontal(1)))
            .style(Style::new()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD))
            .render(version, buf);
    }

    fn render_content(&self, area: Rect, buf: &mut Buffer) {
        let chunks = area.inner(&Margin { horizontal: 1, vertical: 1 });
        let [left, right] = Layout::horizontal([
            Constraint::Length(25),
            Constraint::Min(0),
        ]).areas(chunks);

        Paragraph::new("text").render(left, buf);

        let mut state = TableState::default().with_selected(Some(self.app.context.index));
        let rows = [Row::new(vec!["abc", "def", "ghi"])];

        StatefulWidget::render(
            Table::new(rows, [Constraint::Length(12), Constraint::Length(32)])
            .block(Block::new())
            .header(Row::new(vec!["#", "hash"]).style(Style::new().add_modifier(Modifier::BOLD).add_modifier(Modifier::UNDERLINED))),
            right,
            buf,
            &mut state,
        );


        //Paragraph::new("text").render(right, buf);
    }

    fn render_status_bar(&self, area: Rect, buf: &mut Buffer) {
        let keys = [("Esc", "Exit")];
        let chunks = keys
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(
                    format!(" {} ", key),
                    Style::new().fg(Color::Black).bg(Color::Red)
                );
                let desc = Span::styled(
                    format!(" {} ", desc),
                    Style::new().fg(Color::Red).bg(Color::Black)
                );
                [key, desc]
            })
            .collect_vec();

        Line::from(chunks)
            .centered()
            .render(area, buf);
    }
}
