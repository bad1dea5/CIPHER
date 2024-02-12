//
//
//

use color_eyre::{
    config::HookBuilder,
    eyre::Context,
};
use crossterm::{
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::Backend,
    Terminal,
};
use crate::{
    app::{App, Result},
    event::EventHandler,
    ui::Ui,
};

//
//
//
pub struct Tui<B>
where
    B: Backend
{
    terminal: Terminal<B>,
    pub events: EventHandler,
}

//
//
//
impl<B> Tui<B>
where
    B: Backend + Send + Sync + 'static
{
    pub fn new(terminal: Terminal<B>) -> Result<Self> {
        Ok(Self { 
            terminal,
            events: EventHandler::new(250)?
        })
    }

    pub fn enter(&self) -> Result<()> {
        self.install_hooks()?;

        crossterm::terminal::enable_raw_mode()
            .context("Failed to enable raw mode")?;
        std::io::stdout()
            .execute(EnterAlternateScreen)
            .wrap_err("Failed to EnterAlternateScreen")?;

        Ok(())
    }

    pub fn exit(&self) -> Result<()> {
        crossterm::terminal::disable_raw_mode()
            .context("Failed to disable raw mode")?;
        std::io::stdout()
            .execute(LeaveAlternateScreen)
            .wrap_err("Failed to LeaveAlternateScreen")?;

        Ok(())
    }

    pub fn draw<'a>(&mut self, app: &'a mut App) -> Result<()> {
        self.terminal.draw(|f| f.render_widget(Ui::new(app), f.size()))?;
        Ok(())
    }

    fn install_hooks(&self) -> Result<()> {

        let (panic, error) = HookBuilder::default().into_hooks();
        let panic = panic.into_panic_hook();
        let error = error.into_eyre_hook();

        color_eyre::eyre::set_hook(Box::new(move |e| {
            let _ = crossterm::terminal::disable_raw_mode()
                .context("Failed to disable raw mode");
            let _ = std::io::stdout()
                .execute(LeaveAlternateScreen)
                .wrap_err("Failed to LeaveAlternateScreen");
            error(e)
        }))?;

        std::panic::set_hook(Box::new(move |info| {
            // hack
            let _ = crossterm::terminal::disable_raw_mode()
                .context("Failed to disable raw mode");
            let _ = std::io::stdout()
                .execute(LeaveAlternateScreen)
                .wrap_err("Failed to LeaveAlternateScreen");
            panic(info)
        }));

        Ok(())
    }
}
