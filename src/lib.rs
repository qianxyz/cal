pub type Year = u32;
pub type Month = u8; /* 1 = January, ... */
type Weekday = u8; /* 0 = Sunday, ... */

pub const MONTH_WIDTH: usize = 21;
const DAY_ROWS: u8 = 6;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct YearMonth(Year, Month);

impl YearMonth {
    pub fn new(year: Year, month: Month) -> Self {
        assert!((1..=12).contains(&month));
        Self(year, month)
    }

    fn year(&self) -> Year {
        self.0
    }

    fn month(&self) -> Month {
        self.1
    }

    fn is_leap_year(&self) -> bool {
        match self.year() {
            y if y % 400 == 0 => true,
            y if y % 100 == 0 => false,
            y => y % 4 == 0,
        }
    }

    fn num_of_days(&self) -> u8 {
        match self.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.is_leap_year() {
                    29
                } else {
                    28
                }
            }
            _ => unreachable!(),
        }
    }

    fn weekday_of_first(&self) -> Weekday {
        let a: u32 = (14 - self.month() as u32) / 12;
        let y: u32 = self.year() - a;
        let m: u32 = self.month() as u32 + 12 * a - 2;

        ((1 + y + y / 4 - y / 100 + y / 400 + 31 * m / 12) % 7) as u8
    }

    fn month_header(&self) -> String {
        let smon = match self.month() {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => unreachable!(),
        };

        let header = format!("{} {}", smon, self.year());
        format!("{:^1$}", header, MONTH_WIDTH)
    }

    fn week_header(&self, fday: Weekday) -> String {
        const WEEK_HEADER: [&str; 7] = ["Su ", "Mo ", "Tu ", "We ", "Th ", "Fr ", "Sa "];

        (0..7)
            .map(|i| WEEK_HEADER[(i + fday) as usize % 7])
            .collect::<Vec<_>>()
            .join("")
    }

    fn day_matrix(&self, fday: Weekday) -> impl Iterator<Item = String> {
        let start = 1 - (self.weekday_of_first() as i8 - fday as i8).rem_euclid(7);
        let month_length = self.num_of_days() as i8;
        (0..DAY_ROWS as i8).map(move |n| {
            (start + 7 * n..start + 7 * n + 7)
                .map(|d| {
                    if 1 <= d && d <= month_length {
                        format!("{:>2} ", d)
                    } else {
                        String::from("   ")
                    }
                })
                .collect::<Vec<_>>()
                .join("")
        })
    }

    fn calendar(&self, fday: Weekday) -> Vec<String> {
        let mut cal = Vec::new();
        cal.push(self.month_header());
        cal.push(self.week_header(fday));
        cal.extend(self.day_matrix(fday));

        cal
    }

    fn prev_month(&self) -> Self {
        if self.month() == 1 {
            YearMonth(self.year() - 1, 12)
        } else {
            YearMonth(self.year(), self.month() - 1)
        }
    }

    fn next_month(&self) -> Self {
        if self.month() == 12 {
            YearMonth(self.year() + 1, 1)
        } else {
            YearMonth(self.year(), self.month() + 1)
        }
    }

    fn iter(&self) -> YearMonthIter {
        YearMonthIter(*self)
    }
}

struct YearMonthIter(YearMonth);

impl Iterator for YearMonthIter {
    type Item = YearMonth;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.0;
        self.0 = cur.next_month();
        Some(cur)
    }
}

pub struct Calendar {
    origin: YearMonth,
    len: usize,
    span: bool,
    fday: Weekday,
    column: usize,
}

impl Calendar {
    pub fn new(origin: YearMonth, len: usize, span: bool, fday: Weekday, column: usize) -> Self {
        assert!((0..7).contains(&fday));
        Self {
            origin,
            len,
            span,
            fday,
            column,
        }
    }

    fn months(&self) -> impl Iterator<Item = YearMonth> {
        let mut start = self.origin;
        if self.span {
            for _ in 0..self.len / 2 {
                start = start.prev_month();
            }
        }
        start.iter().take(self.len)
    }

    fn format(&self) -> String {
        self.months()
            .map(|ym| ym.calendar(self.fday))
            .collect::<Vec<_>>()
            .chunks(self.column)
            .map(|cs| {
                (0..cs[0].len())
                    .map(|i| {
                        cs.iter()
                            .map(|c| c[i].as_str())
                            .collect::<Vec<_>>()
                            .join(" ")
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl std::fmt::Display for Calendar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leap_year() {
        assert!(YearMonth(2000, 1).is_leap_year());
        assert!(YearMonth(2020, 1).is_leap_year());
        assert!(!YearMonth(2021, 1).is_leap_year());
        assert!(!YearMonth(2100, 1).is_leap_year());
    }

    #[test]
    fn month_length() {
        assert_eq!(YearMonth(2020, 1).num_of_days(), 31);
        assert_eq!(YearMonth(2020, 2).num_of_days(), 29);
        assert_eq!(YearMonth(2020, 4).num_of_days(), 30);
        assert_eq!(YearMonth(2021, 2).num_of_days(), 28);
    }

    #[test]
    fn weekday_of_first() {
        assert_eq!(YearMonth(2022, 11).weekday_of_first(), 2);
        assert_eq!(YearMonth(2022, 12).weekday_of_first(), 4);
        assert_eq!(YearMonth(2023, 1).weekday_of_first(), 0);
        assert_eq!(YearMonth(2023, 2).weekday_of_first(), 3);
    }

    #[test]
    fn month_header() {
        assert_eq!(YearMonth(2022, 1).month_header(), "    January 2022     ");
        assert_eq!(YearMonth(2022, 2).month_header(), "    February 2022    ");
        assert_eq!(YearMonth(2022, 3).month_header(), "     March 2022      ");
    }

    #[test]
    fn week_header() {
        assert_eq!(YearMonth(2022, 1).week_header(0), "Su Mo Tu We Th Fr Sa ");
        assert_eq!(YearMonth(2022, 1).week_header(1), "Mo Tu We Th Fr Sa Su ");
        assert_eq!(YearMonth(2022, 1).week_header(6), "Sa Su Mo Tu We Th Fr ");
    }

    #[test]
    fn prev_next_month() {
        assert_eq!(YearMonth(2022, 11).prev_month(), YearMonth(2022, 10));
        assert_eq!(YearMonth(2022, 11).next_month(), YearMonth(2022, 12));
        assert_eq!(YearMonth(2022, 1).prev_month(), YearMonth(2021, 12));
        assert_eq!(YearMonth(2022, 12).next_month(), YearMonth(2023, 1));
    }

    #[test]
    fn iter_month() {
        let mut iter = YearMonth(2021, 12).iter();
        assert_eq!(iter.next(), Some(YearMonth(2021, 12)));
        for m in 1..=12 {
            assert_eq!(iter.next(), Some(YearMonth(2022, m)));
        }
        assert_eq!(iter.next(), Some(YearMonth(2023, 1)));
    }

    #[test]
    fn calendar_vec() {
        assert_eq!(
            YearMonth(2022, 11).calendar(0),
            [
                "    November 2022    ",
                "Su Mo Tu We Th Fr Sa ",
                "       1  2  3  4  5 ",
                " 6  7  8  9 10 11 12 ",
                "13 14 15 16 17 18 19 ",
                "20 21 22 23 24 25 26 ",
                "27 28 29 30          ",
                "                     "
            ]
        );
    }

    #[test]
    fn calendar_vec_with_fday() {
        assert_eq!(
            YearMonth(2022, 11).calendar(1),
            [
                "    November 2022    ",
                "Mo Tu We Th Fr Sa Su ",
                "    1  2  3  4  5  6 ",
                " 7  8  9 10 11 12 13 ",
                "14 15 16 17 18 19 20 ",
                "21 22 23 24 25 26 27 ",
                "28 29 30             ",
                "                     "
            ]
        );
        assert_eq!(
            YearMonth(2022, 11).calendar(3),
            [
                "    November 2022    ",
                "We Th Fr Sa Su Mo Tu ",
                "                   1 ",
                " 2  3  4  5  6  7  8 ",
                " 9 10 11 12 13 14 15 ",
                "16 17 18 19 20 21 22 ",
                "23 24 25 26 27 28 29 ",
                "30                   "
            ]
        );
    }

    #[test]
    fn month_iter() {
        let cal = Calendar::new(YearMonth(2022, 11), 3, false, 0, 1);
        let mut iter = cal.months();
        assert_eq!(iter.next(), Some(YearMonth(2022, 11)));
        assert_eq!(iter.next(), Some(YearMonth(2022, 12)));
        assert_eq!(iter.next(), Some(YearMonth(2023, 1)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn month_iter_with_span() {
        let cal = Calendar::new(YearMonth(2022, 11), 3, true, 0, 1);
        let mut iter = cal.months();
        assert_eq!(iter.next(), Some(YearMonth(2022, 10)));
        assert_eq!(iter.next(), Some(YearMonth(2022, 11)));
        assert_eq!(iter.next(), Some(YearMonth(2022, 12)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn draw_single_month() {
        assert_eq!(
            Calendar::new(YearMonth(2022, 11), 1, false, 0, 1).format(),
            "\
\x20   November 2022    \n\
   Su Mo Tu We Th Fr Sa \n\
\x20      1  2  3  4  5 \n\
\x206  7  8  9 10 11 12 \n\
   13 14 15 16 17 18 19 \n\
   20 21 22 23 24 25 26 \n\
   27 28 29 30          \n\
\x20                    "
        );
    }

    #[test]
    fn draw_two_months() {
        assert_eq!(
            Calendar::new(YearMonth(2022, 11), 2, false, 0, 3).format(),
            "\
\x20   November 2022         December 2022    \n\
   Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa \n\
\x20      1  2  3  4  5               1  2  3 \n\
\x206  7  8  9 10 11 12   4  5  6  7  8  9 10 \n\
   13 14 15 16 17 18 19  11 12 13 14 15 16 17 \n\
   20 21 22 23 24 25 26  18 19 20 21 22 23 24 \n\
   27 28 29 30           25 26 27 28 29 30 31 \n\
\x20                                          "
        );
    }

    #[test]
    fn draw_year() {
        assert_eq!(
            Calendar::new(YearMonth(2022, 1), 12, false, 0, 3).format(),
            "\
\x20   January 2022          February 2022          March 2022      \n\
   Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa \n\
\x20                  1         1  2  3  4  5         1  2  3  4  5 \n\
\x202  3  4  5  6  7  8   6  7  8  9 10 11 12   6  7  8  9 10 11 12 \n\
\x209 10 11 12 13 14 15  13 14 15 16 17 18 19  13 14 15 16 17 18 19 \n\
   16 17 18 19 20 21 22  20 21 22 23 24 25 26  20 21 22 23 24 25 26 \n\
   23 24 25 26 27 28 29  27 28                 27 28 29 30 31       \n\
   30 31                                                            \n\
\x20    April 2022             May 2022              June 2022      \n\
   Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa \n\
\x20               1  2   1  2  3  4  5  6  7            1  2  3  4 \n\
\x203  4  5  6  7  8  9   8  9 10 11 12 13 14   5  6  7  8  9 10 11 \n\
   10 11 12 13 14 15 16  15 16 17 18 19 20 21  12 13 14 15 16 17 18 \n\
   17 18 19 20 21 22 23  22 23 24 25 26 27 28  19 20 21 22 23 24 25 \n\
   24 25 26 27 28 29 30  29 30 31              26 27 28 29 30       \n\
\x20                                                                \n\
\x20     July 2022            August 2022         September 2022    \n\
   Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa \n\
\x20               1  2      1  2  3  4  5  6               1  2  3 \n\
\x203  4  5  6  7  8  9   7  8  9 10 11 12 13   4  5  6  7  8  9 10 \n\
   10 11 12 13 14 15 16  14 15 16 17 18 19 20  11 12 13 14 15 16 17 \n\
   17 18 19 20 21 22 23  21 22 23 24 25 26 27  18 19 20 21 22 23 24 \n\
   24 25 26 27 28 29 30  28 29 30 31           25 26 27 28 29 30    \n\
   31                                                               \n\
\x20   October 2022          November 2022         December 2022    \n\
   Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa  Su Mo Tu We Th Fr Sa \n\
\x20                  1         1  2  3  4  5               1  2  3 \n\
\x202  3  4  5  6  7  8   6  7  8  9 10 11 12   4  5  6  7  8  9 10 \n\
\x209 10 11 12 13 14 15  13 14 15 16 17 18 19  11 12 13 14 15 16 17 \n\
   16 17 18 19 20 21 22  20 21 22 23 24 25 26  18 19 20 21 22 23 24 \n\
   23 24 25 26 27 28 29  27 28 29 30           25 26 27 28 29 30 31 \n\
   30 31                                                            "
        );
    }
}
