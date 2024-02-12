//
//
//

use std::fmt::Display;

static VERSION: u64 = 0x0000_0001_0000_0000;

#[derive(Clone, Copy, Debug)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16
}

impl Default for Version {
    fn default() -> Self {
        Self {
            major: (VERSION >> 48) as u16,
            minor: (VERSION >> 32) as u16,
            patch: (VERSION >> 16) as u16,
        }
    }
}

impl Version {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn as_string(self) -> String {
        fn inner<D, I>(version: I) -> String
        where
            D: Display,
            I: IntoIterator<Item=D>
        {
            let mut iterator = version.into_iter();
            let begin = match iterator.next() {
                Some(r) => format!("{r}"),
                None => String::new()
            };
            iterator.fold(begin, |a, x| format!("{a}.{x}"))
        }
        inner(vec![self.major, self.minor, self.patch])
    }
}
