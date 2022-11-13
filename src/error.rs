use std::fmt;

#[derive(Debug)]
pub enum CalError {
    InvalidMonth(u8),
    InvalidWeekday(u8),
}

impl fmt::Display for CalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMonth(val) => {
                write!(f, "invalid month: {}", val)
            }
            Self::InvalidWeekday(val) => {
                write!(f, "invalid weekday: {}", val)
            }
        }
    }
}

impl std::error::Error for CalError {}

pub type CalResult<T> = Result<T, CalError>;
