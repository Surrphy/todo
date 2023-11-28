use std::fmt::{
    self,
    Display
};

pub enum Colors {
    Red,
    Green,
    Blue,
    Clear
}

impl Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Red => write!(f, "\x1b[91m"),
            Self::Green => write!(f, "\x1b[92m"),
            Self::Blue => write!(f, "\x1b[94m"),
            Self::Clear => write!(f, "\x1b[39m")
        }
    }
}
