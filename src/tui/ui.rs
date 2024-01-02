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

    let formatted_projects = app
        .projects
        .iter()
        .map(|project| {
            let time = project.format_time();

            let padding = " ".repeat(6 - time.to_string().len());

            format!("{}{} - {}", padding, time, project.get_path())
        })
        .collect::<Vec<String>>();

    f.render_stateful_widget(
        List::new(formatted_projects)
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
