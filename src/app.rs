//
//
//

use crossterm::event::{KeyCode, KeyEvent, MouseEvent};
use std::{
    error::Error,
    result::Result as IoResult,
};
use crate::version::Version;

pub type Result<T> = IoResult<T, Box<dyn Error>>;

//
//
//
#[derive(Debug, Default, Clone, Copy)]
pub struct AppContext {
    pub index: usize,
}

//
//
//
#[allow(dead_code)]
#[derive(Debug)]
pub struct App<'a> {
    pub context: AppContext,
    pub name: &'a str,
    pub is_running: bool,
    version: Version,
}

//
//
//
impl<'a> App<'a> {
    pub fn new(name: &'a str) -> Result<Self> {
        Ok(Self {
            context: AppContext::default(),
            name,
            is_running: true,
            version: Version::default(),
        })
    }

    pub async fn handle_key_events(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.exit();
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_mouse_events(&self, _: MouseEvent) -> Result<()> {
        Ok(())
    }

    pub fn exit(&mut self) {
        self.is_running = false
    }

    pub fn version(&mut self) -> Version {
        self.version
    }
}
