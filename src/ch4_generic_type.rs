//!
//! Find this code at:
//! https://github.com/maciejhirsz/sub0_examples
//!
use std::fmt::{self, Display, Write};

pub struct Hello<T> {
    what: T,
}

impl Hello {
    pub fn new(what: T) -> Self {
        Hello { what }
    }
}

impl Display for Hello {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Hello, {}!", self.what)
    }
}