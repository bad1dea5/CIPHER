//
//
//

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::{Alignment, Frame, Line, Span},
    style::*, widgets::*
};

use crate::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(Rect::new(
            0,
            0,
            f.size().width,
            f.size().height
        )
    );

    f.render_widget(
        Block::new()
            .style(Style::new()
                .bg(Color::Black)
                .fg(Color::Red)
            ),
        f.size()
    );

    render_title_bar(app, vertical[0], f);
    render_content(app, vertical[1], f);
    render_status_bar(app, vertical[2], f);
}

fn render_title_bar(app: &mut App, area: Rect, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(area);

    f.render_widget(
        Block::default()
            .title(
                app.name.to_ascii_uppercase()
            )
            .style(Style::new()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
            ),
        layout[0]
    );

    f.render_widget(
        Block::default()
            .title(format!("{}", app.version()))
            .title_alignment(Alignment::Right)
            .style(Style::new()
                .bg(Color::Red)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
            ),
        layout[1]
    );
}

fn render_content(_app: &mut App, area: Rect, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Min(0)
        ])
        .split(area);

    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default()),
        layout[0]
    );

    //
    let mut state = TableState::new().with_selected(Some(0));
    let rows = [
        Row::new(vec!["1", "0x0000000000000000"]),
        Row::new(vec!["2", "0x0000000000000000"]),
        Row::new(vec!["3", "0x0000000000000000"]),
    ];

    f.render_stateful_widget(
        Table::new(
            rows,
            [
                Constraint::Length(30),
                Constraint::Min(30)
            ])
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .header(Row::new(vec!["Id", "Cipher"])
                .style(Style::new()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::UNDERLINED)
            )
            .bottom_margin(1)),
        Rect::new(
            layout[1].x + 1,
            layout[1].y + 1,
            layout[1].width - 2,
            layout[1].height - 2),
        &mut state
    );

}

fn render_status_bar(_app: &mut App, area: Rect, f: &mut Frame) {
    let text = vec![
        Line::from(vec![
            Span::styled(format!(" {} ", "↓"), Style::new().fg(Color::Black).bg(Color::Red)),
            Span::styled(format!(" {} ", "Next"), Style::new().fg(Color::Red).bg(Color::Black)),
            Span::styled(format!(" {} ", "↑"), Style::new().fg(Color::Black).bg(Color::Red)),
            Span::styled(format!(" {} ", "Prev"), Style::new().fg(Color::Red).bg(Color::Black)),
            Span::styled(format!(" {} ", "Esc"), Style::new().fg(Color::Black).bg(Color::Red)),
            Span::styled(format!(" {} ", "Quit"), Style::new().fg(Color::Red).bg(Color::Black)),
        ]),
    ];

    f.render_widget(
        Paragraph::new(text)
            .alignment(Alignment::Center),
        area
    );
}
