//
//
//

use crate::version::Version;

#[derive(Debug, Default)]
pub struct App<'a> {
    pub name: &'a str,
    pub version: Version,
    pub is_running: bool,
}

impl<'a> App<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name: name,
            version: Version::new(),
            is_running: true,
        }
    }

    pub fn quit(&mut self) {
        self.is_running = false;
    }

    pub fn version(&self) -> String {
        self.version.show()
    }
}
