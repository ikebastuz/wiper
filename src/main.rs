use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::env;
use std::io;
use std::process;
use wiper::app::{App, AppResult};
use wiper::config::InitConfig;
use wiper::config::EVENT_INTERVAL;
use wiper::events::{handle_key_events, Event, EventHandler};
use wiper::fs::DSHashmap;
use wiper::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = InitConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut app: App<DSHashmap> = App::new(config);
    app.init();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(EVENT_INTERVAL);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app).await?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    tui.exit()?;
    Ok(())
}
