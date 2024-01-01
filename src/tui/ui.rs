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
    
    f.render_stateful_widget(
        List::new(app.repositories.clone())
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .highlight_symbol(">>")
            .repeat_highlight_symbol(true),
        layout[0],
        &mut app.items,
    );

    f.render_widget(
        Paragraph::new(format!(
            "
        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
        Press `j` and `k` to increment and decrement the counter respectively.\n\
        Counter: {}
      ",
            app.counter
        ))
        .block(
            Block::default()
                .title("Counter App")
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center),
        layout[1],
    )
}
