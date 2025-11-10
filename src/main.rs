mod app;
mod events;
mod ui;
use app::App;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use events::handle_input;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, time::Duration};
use ui::draw_ui;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let mut app = App::new();

    loop {
        terminal.draw(|frame| draw_ui(frame, &app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                handle_input(&mut app, key_event);
            }
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        disable_raw_mode().ok();
        let mut stdout = std::io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture).ok();
    }
}
