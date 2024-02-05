//
//
//

mod app;
mod event;
mod tui;
mod ui;
mod version;

use event::Event;
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};
use crate::{
    app::{App, Result},
    tui::Tui,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new("cipher")?;
    let terminal = Terminal::new(
        CrosstermBackend::new(std::io::stdout())
    )?;
    let mut tui = Tui::new(terminal)?;

    tui.enter()?;

    while app.is_running {
        tui.draw()?;

        match tui.events.next().await? {
            Event::Key(key) => {
                app.handle_key_events(key).await?
            }
            Event::Mouse(mouse) => {
                app.handle_mouse_events(mouse).await?
            }
            Event::Resize(_, _) => {}
            Event::Tick => {}
        }
    }

    tui.exit()?;

    Ok(())
}
