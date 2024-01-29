//
//
//

use std::fmt::Display;

static VERSION: u64 = 0x0000_0001_0000_0000;

#[derive(Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16
}

impl Default for Version {
    fn default() -> Self {
        Version {
            major: (VERSION >> 48) as u16,
            minor: (VERSION >> 32) as u16,
            patch: (VERSION >> 16) as u16
        }
    }
}

impl Version {
    pub fn new() -> Version {
        Version {
            major: (VERSION >> 48) as u16,
            minor: (VERSION >> 32) as u16,
            patch: (VERSION >> 16) as u16
        }
    }

    pub fn show(&self) -> String {
        let version: Vec<u16> = vec![self.major, self.minor, self.patch];

        fn inner<D, I>(array: I) -> String
        where
            D: Display,
            I: IntoIterator<Item=D>
        {
            let mut iterator = array.into_iter();
            let head = match iterator.next() {
                Some(r) => format!("{r}"),
                None => String::new()
            };

            iterator.fold(head, |acc, x| format!("{acc}.{x}"))
        }
        
        inner(version)
    }
}
