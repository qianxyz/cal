use std::convert;
use std::fmt;

use crate::error::{CalError, CalResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Year(u32);

impl Year {
    pub fn pred(&self) -> Self {
        (self.0 - 1).into()
    }

    pub fn succ(&self) -> Self {
        (self.0 + 1).into()
    }

    pub fn is_leap_year(&self) -> bool {
        match self.0 {
            y if y % 400 == 0 => true,
            y if y % 100 == 0 => false,
            y => y % 4 == 0,
        }
    }
}

impl convert::From<u32> for Year {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl convert::From<Year> for u32 {
    fn from(year: Year) -> Self {
        year.0
    }
}

impl fmt::Display for Year {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn pred(&self) -> Self {
        match self {
            Self::January => Self::December,
            _ => (*self as u8 - 1).try_into().unwrap(),
        }
    }

    pub fn succ(&self) -> Self {
        match self {
            Self::December => Self::January,
            _ => (*self as u8 + 1).try_into().unwrap(),
        }
    }
}

impl convert::TryFrom<u8> for Month {
    type Error = CalError;

    fn try_from(value: u8) -> CalResult<Self> {
        match value {
            1 => Ok(Self::January),
            2 => Ok(Self::February),
            3 => Ok(Self::March),
            4 => Ok(Self::April),
            5 => Ok(Self::May),
            6 => Ok(Self::June),
            7 => Ok(Self::July),
            8 => Ok(Self::August),
            9 => Ok(Self::September),
            10 => Ok(Self::October),
            11 => Ok(Self::November),
            12 => Ok(Self::December),
            _ => Err(CalError::InvalidMonth(value)),
        }
    }
}

impl fmt::Display for Month {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Weekday {
    Sunday = 0,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl convert::TryFrom<u8> for Weekday {
    type Error = CalError;

    fn try_from(value: u8) -> CalResult<Self> {
        match value {
            0 => Ok(Self::Sunday),
            1 => Ok(Self::Monday),
            2 => Ok(Self::Tuesday),
            3 => Ok(Self::Wednesday),
            4 => Ok(Self::Thursday),
            5 => Ok(Self::Friday),
            6 => Ok(Self::Saturday),
            _ => Err(CalError::InvalidWeekday(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pred_succ_year() {
        assert_eq!(Year(2022).pred(), Year(2021));
        assert_eq!(Year(2022).succ(), Year(2023));
    }

    #[test]
    fn leap_year() {
        assert!(Year(2000).is_leap_year());
        assert!(Year(2020).is_leap_year());
        assert!(!Year(2021).is_leap_year());
        assert!(!Year(2100).is_leap_year());
    }

    #[test]
    fn display_year() {
        assert_eq!(Year(1).to_string(), "1");
        assert_eq!(Year(2020).to_string(), "2020");
    }

    #[test]
    fn new_month() {
        assert_eq!(Month::try_from(1).unwrap(), Month::January);
        assert!(Month::try_from(13).is_err());
    }

    #[test]
    fn pred_succ_month() {
        assert_eq!(Month::January.pred(), Month::December);
        assert_eq!(Month::December.pred(), Month::November);
        assert_eq!(Month::December.succ(), Month::January);
        assert_eq!(Month::January.succ(), Month::February);
    }

    #[test]
    fn display_month() {
        assert_eq!(Month::January.to_string(), "January");
        assert_eq!(Month::February.to_string(), "February");
    }

    #[test]
    fn new_weekday() {
        assert_eq!(Weekday::try_from(0).unwrap(), Weekday::Sunday);
        assert!(Weekday::try_from(7).is_err());
    }
}
