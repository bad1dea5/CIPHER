//
//
//

mod app;
mod event;
mod ui;
mod tui;
mod update;
mod version;

use color_eyre::Result;
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};

use app::App;
use event::{Event, EventHandler};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    let mut app = App::new("cipher");

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    tui.enter()?;

    while app.is_running {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(key_event) => update(&mut app, key_event),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;

    Ok(())
}