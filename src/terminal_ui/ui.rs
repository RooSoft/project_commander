use ratatui::{
    prelude::{Alignment, Constraint, Direction, Frame, Layout, Modifier},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use fuse_rust::{Fuse, ScoreResult};

use crate::terminal_ui::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(get_constraints(app))
        .split(f.size());

    app.items.select(app.items.selected());
    app.tick();

    let formatted_projects = app
        .display_projects
        .iter()
        .map(|p| p.to_string())
        .filter(|p| {
            if app.search_text.is_empty() {
                true
            } else {
                if let Some(ScoreResult { score, ranges: _ }) =
                    Fuse::default().search_text_in_string(&app.search_text[..], p)
                {
                    score > 0.3
                } else {
                    false
                }
            }
        })
        .collect::<Vec<String>>();

    let search_text = format!(" {}", &app.search_text);

    if app.searching {
        f.render_widget(
            Paragraph::new(search_text)
                .block(
                    Block::default()
                        .title(" Search ")
                        .title_alignment(Alignment::Left)
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded),
                )
                .style(Style::default().fg(Color::Yellow)),
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
    if app.searching {
        vec![Constraint::Length(3), Constraint::Percentage(50)]
    } else {
        vec![Constraint::Length(0), Constraint::Percentage(50)]
    }
}
