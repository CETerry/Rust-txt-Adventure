use ratatui::{
    layout::Alignment,
    // style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
    prelude::*,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(3),
                // Constraint::Min(0), // Allow empty space
            ]
            .as_ref(),
        )
        .split(frame.size());
    frame.render_widget(
        Paragraph::new(app.input.as_str())
            .block(
                Block::default()
                    .title("Input")
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(app.output.as_str())
            .block(
                Block::default()
                    .title("Output") // TODO: Come up with a better title
                    .title_alignment(Alignment::Left)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            ),
            chunks[1],
    );

    frame.set_cursor(
        chunks[0].x + app.cursor_position as u16 + 1,
        chunks[0].y + 1
    )
}
