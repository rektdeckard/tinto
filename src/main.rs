use dotenv::dotenv;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tinto::app::{App, AppResult};
use tinto::event::{Event, EventHandler};
use tinto::handler::handle_key_events;
use tinto::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    // Create an application.
    let mut app = App::with_discovery().await;
    let _ = &app.bridge.refresh().await.unwrap();

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
            Event::Mouse(_mouse_event) => {
                // app.counter = (mouse_event.column / 2) as u8;
                // if mouse_event.row == 0 {
                //     app.bridge
                //         .group("d14bacd9-a352-4f90-912b-6e6f272ff059")
                //         .unwrap()
                //         .send(&[hues::GroupCommand::Signaling {
                //             signal: hues::SignalType::Alternating,
                //             duration: 8000,
                //             colors: Some(hues::SignalColor::Two(
                //                 hues::CIEColor::from_hex("#00BADD").unwrap(),
                //                 hues::CIEColor::from_hex("#FAFACE").unwrap(),
                //             )),
                //         }])
                //         .await
                //         .unwrap();
                // }
            }
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
