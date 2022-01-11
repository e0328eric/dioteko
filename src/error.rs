use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy)]
pub enum DiotekoErr {
    WindowInitFailedErr,
    LoadImageFailedErr,
}

impl Display for DiotekoErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowInitFailedErr => write!(f, "Failed to initialize window"),
            Self::LoadImageFailedErr => write!(f, "Failed to load an image"),
        }
    }
}

pub type Result<T> = std::result::Result<T, DiotekoErr>;
