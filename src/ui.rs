use ratatui::{
    layout::Alignment,
    // style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, List, ListItem, Wrap},
    Frame,
    prelude::*,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    app.clamp_inventory();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Length(3),
                Constraint::Percentage(50),
                // Constraint::Min(0), // Allow empty space
            ]
            .as_ref(),
        )
        .split(frame.size());
    frame.render_widget(
        Paragraph::new(app.input.as_str())
            .block(
                Block::default()
                    .title(app.backend.get_location())
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new(app.output.as_str())
            .block(
                Block::default()
                    .title("Output") // TODO: Come up with a better title
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true }),
            chunks[0],
    );
    frame.set_cursor(
        chunks[1].x + app.cursor_position as u16 + 1,
        chunks[1].y + 1
    );
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(chunks[2]);
    let inventory = app.backend.get_inventory();
    let items: Vec<ListItem> = inventory.iter().map(|i| ListItem::new(i.as_str())).collect();
    let items = List::new(items)
        .block(
            Block::default()
                .title(format!("Inventory ({})",inventory.len()))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .highlight_style(Style::default().fg(Color::Yellow))
        .highlight_symbol(">> ");
    frame.render_stateful_widget(items, chunks[1], &mut app.inventory_state);

    let exits = app.backend.get_exits();
    frame.render_widget(
        Paragraph::new(exits.join("\n").as_str())
            .block(
                Block::default()
                    .title("Exits")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(Wrap { trim: true }),
        chunks[0]
    );
}
