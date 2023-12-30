use crossterm::{
    event::{self, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{CrosstermBackend, Frame, Terminal},
    widgets::Paragraph,
};

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

struct App {
    counter: i64,
    should_quit: bool,
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stderr(), EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    execute!(std::io::stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(app: &App, f: &mut Frame) {
    f.render_widget(
        Paragraph::new(format!("Counter: {}", app.counter)),
        f.size(),
    );
}

fn update(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(250))? {
        if let Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    Char('j') => app.counter += 1,
                    Char('k') => app.counter -= 1,
                    Char('q') => app.should_quit = true,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}

fn run() -> Result<()> {
    // Initialize the terminal backend using crossterm
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stderr()))?;

    // Define our counter variable
    // This is the state of our application
    let mut app = App {
        counter: 0,
        should_quit: false,
    };

    // Main application loop
    loop {
        // Render the UI
        terminal.draw(|f| ui(&app, f))?;

        // Check for user input every 250 milliseconds
        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    startup()?;

    let status = run();

    shutdown()?;

    status?;

    Ok(())
}
