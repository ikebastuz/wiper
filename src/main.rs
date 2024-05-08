use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use space_inspector::app::{App, AppResult};
use space_inspector::config::InitConfig;
use space_inspector::event::{Event, EventHandler};
use space_inspector::handler::handle_key_events;
use space_inspector::tui::Tui;
use std::env;
use std::io;
use std::process;

#[tokio::main]
async fn main() -> AppResult<()> {
    let config = InitConfig::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let mut app = App::new(config);
    app.init();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
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
