use crate::error::CalResult;
use crate::wrapper::{Month, Weekday, Year};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MonthOfYear(Year, Month);

impl MonthOfYear {
    pub fn new(year: u32, month: u8) -> CalResult<Self> {
        Ok(Self(year.into(), month.try_into()?))
    }

    pub fn year(&self) -> Year {
        self.0
    }

    pub fn month(&self) -> Month {
        self.1
    }

    fn num_of_days(&self) -> u8 {
        use Month::*;

        match self.month() {
            January | March | May | July | August | October | December => 31,
            April | June | September | November => 30,
            February => {
                if self.year().is_leap_year() {
                    29
                } else {
                    28
                }
            }
        }
    }

    fn weekday_of_first(&self) -> Weekday {
        let a: u32 = (14 - self.month() as u32) / 12;
        let y: u32 = u32::from(self.year()) - a;
        let m: u32 = self.month() as u32 + 12 * a - 2;

        (((1 + y + y / 4 - y / 100 + y / 400 + 31 * m / 12) % 7) as u8)
            .try_into()
            .unwrap()
    }

    fn pred(&self) -> Self {
        MonthOfYear(
            match self.month() {
                Month::January => self.year().pred(),
                _ => self.year(),
            },
            self.month().pred(),
        )
    }

    fn succ(&self) -> Self {
        MonthOfYear(
            match self.month() {
                Month::December => self.year().succ(),
                _ => self.year(),
            },
            self.month().succ(),
        )
    }

    fn iter(&self) -> MOYIter {
        MOYIter(*self)
    }
}

struct MOYIter(MonthOfYear);

impl Iterator for MOYIter {
    type Item = MonthOfYear;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.0;
        self.0 = cur.succ();
        Some(cur)
    }
}

pub struct CalRange {
    /// the originally requested month of year
    origin: MonthOfYear,

    /// number of consecutive months
    len: usize,

    /// whether to span the origin month
    span: bool,
}

impl CalRange {
    fn new(year: u32, month: u8, len: usize, span: bool) -> CalResult<Self> {
        Ok(Self {
            origin: MonthOfYear::new(year, month)?,
            len,
            span,
        })
    }

    fn iter(&self) -> impl Iterator<Item = MonthOfYear> {
        let mut start = self.origin;
        if self.span {
            for _ in 0..self.len / 2 {
                start = start.pred();
            }
        }
        start.iter().take(self.len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moy(y: u32, m: u8) -> MonthOfYear {
        MonthOfYear::new(y, m).unwrap()
    }

    #[test]
    fn new_moy() {
        assert!(MonthOfYear::new(2022, 1).is_ok());
        assert!(MonthOfYear::new(2022, 13).is_err());
    }

    #[test]
    fn num_of_days() {
        assert_eq!(moy(2020, 1).num_of_days(), 31);
        assert_eq!(moy(2020, 2).num_of_days(), 29);
        assert_eq!(moy(2020, 4).num_of_days(), 30);
        assert_eq!(moy(2021, 2).num_of_days(), 28);
    }

    #[test]
    fn weekday_of_first() {
        use Weekday::*;

        assert_eq!(moy(2022, 11).weekday_of_first(), Tuesday);
        assert_eq!(moy(2022, 12).weekday_of_first(), Thursday);
        assert_eq!(moy(2023, 1).weekday_of_first(), Sunday);
        assert_eq!(moy(2023, 2).weekday_of_first(), Wednesday);
    }

    #[test]
    fn pred_succ() {
        assert_eq!(moy(2022, 11).pred(), moy(2022, 10));
        assert_eq!(moy(2022, 11).succ(), moy(2022, 12));
        assert_eq!(moy(2022, 1).pred(), moy(2021, 12));
        assert_eq!(moy(2022, 12).succ(), moy(2023, 1));
    }

    #[test]
    fn calrange_iter() {
        let cal = CalRange::new(2022, 11, 3, false).unwrap();
        let mut iter = cal.iter();
        assert_eq!(iter.next(), Some(moy(2022, 11)));
        assert_eq!(iter.next(), Some(moy(2022, 12)));
        assert_eq!(iter.next(), Some(moy(2023, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn calrange_span_iter() {
        let cal = CalRange::new(2022, 11, 4, true).unwrap();
        let mut iter = cal.iter();
        assert_eq!(iter.next(), Some(moy(2022, 9)));
        assert_eq!(iter.next(), Some(moy(2022, 10)));
        assert_eq!(iter.next(), Some(moy(2022, 11)));
        assert_eq!(iter.next(), Some(moy(2022, 12)));
        assert_eq!(iter.next(), None);
    }
}
