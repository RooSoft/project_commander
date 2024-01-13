use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Modifier},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use crate::terminal_ui::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(get_constraints(app))
        .split(f.size());

    app.items.select(app.items.selected());
    app.tick();

    let formatted_projects = app
        .projects
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<String>>();

    if app.show_search {
        f.render_widget(
            Paragraph::new("".to_string())
                .block(
                    Block::default()
                        .title(" Search ")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center),
            layout[0],
        )
    }

    f.render_stateful_widget(
        List::new(formatted_projects)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        layout[1],
        &mut app.items,
    );
}

fn get_constraints(app: &App) -> Vec<Constraint> {
    if app.show_search {
        vec![Constraint::Length(3), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Length(0), Constraint::Percentage(50)]
    }
}
