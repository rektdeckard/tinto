use clap::Parser;
use dotenv::dotenv;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use tinto::{
    app::{App, AppResult, Args},
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();

    // Create an application.
    let args = Args::parse();
    let mut app = App::try_init(args).await?;
    let _ = &app.bridge.refresh().await;

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_mouse_event) => {}
            Event::Resize(_w, _h) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
