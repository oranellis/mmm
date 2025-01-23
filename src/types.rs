use std::ops::{Add, Sub};

// Vec2d

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2d {
    pub col: u16,
    pub row: u16,
}

impl Add for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl Sub for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

impl From<(u16, u16)> for Vec2d {
    fn from(value: (u16, u16)) -> Self {
        Vec2d {
            col: value.0,
            row: value.1,
        }
    }
}

// Error and Result type

#[derive(Debug)]
pub enum MmmError {
    Layout,
    TerminalBuffer,
    Io(std::io::Error),
}

impl std::fmt::Display for MmmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MmmError::Layout => write!(f, "layout error"),
            MmmError::TerminalBuffer => write!(f, "terminal buffer error"),
            MmmError::Io(err) => write!(f, "i/o error: {}", err),
        }
    }
}

impl std::error::Error for MmmError {}

impl From<std::io::Error> for MmmError {
    fn from(error: std::io::Error) -> Self {
        MmmError::Io(error)
    }
}

pub type MmmResult<T> = Result<T, MmmError>;
