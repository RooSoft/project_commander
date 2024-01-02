use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Modifier},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use crate::tui::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(f.size());

    app.items.select(app.items.selected());
    app.tick();

    let formatted_repositories = app
        .repositories
        .iter()
        .map(|(path, timestamp)| {
            let duration = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!")
                .checked_sub(std::time::Duration::from_secs(*timestamp as u64))
                .expect("Duration calculation failed");

            let formatted_duration = format_duration(duration);

            let padding = " ".repeat(6 - formatted_duration.to_string().len());

            format!("{}{} - {}", padding, formatted_duration, path)
        })
        .collect::<Vec<String>>();

    f.render_stateful_widget(
        List::new(formatted_repositories)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        layout[0],
        &mut app.items,
    );

    f.render_widget(
        Paragraph::new(format!("here will lie some repository information"))
            .block(
                Block::default()
                    .title("Repository info")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center),
        layout[1],
    )
}

fn format_duration(duration: std::time::Duration) -> String {
    if duration.as_secs() < 60 {
        // Less than a minute
        format!("{}s", duration.as_secs())
    } else if duration.as_secs() < 3600 {
        // Less than an hour
        format!("{}m", duration.as_secs() / 60)
    } else if duration.as_secs() < 86400 {
        // Less than a day
        format!("{}h", duration.as_secs() / 3600)
    } else {
        // More than a day
        format!("{}d", duration.as_secs() / 86400)
    }
}
