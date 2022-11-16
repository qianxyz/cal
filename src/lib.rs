use chrono::{Datelike, Month, Months, NaiveDate, Weekday};
use colored::Colorize;
use itertools::Itertools;
use num_traits::cast::FromPrimitive;

pub const MONTH_WIDTH: usize = 3 * 7;
const DAY_ROWS: usize = 6;
const MONTH_ROWS: usize = DAY_ROWS + 2;

/// A line like "    November 2022    ".
fn month_year_line(date: NaiveDate) -> String {
    let month = Month::from_u32(date.month()).unwrap();
    let header = format!("{} {}", month.name(), date.year());
    format!("{:^1$}", header, MONTH_WIDTH)
}

/// A cell like "Su" or "Mo".
fn weekday_cell(weekday: Weekday) -> String {
    match weekday {
        Weekday::Mon => "Mo".to_string(),
        Weekday::Tue => "Tu".to_string(),
        Weekday::Wed => "We".to_string(),
        Weekday::Thu => "Th".to_string(),
        Weekday::Fri => "Fr".to_string(),
        Weekday::Sat => "Sa".red().to_string(),
        Weekday::Sun => "Su".red().to_string(),
    }
}

/// A line like "Su Mo Tu We Th Fr Sa ", starting at `start`.
fn weekday_line(start: Weekday) -> String {
    itertools::iterate(start, Weekday::succ)
        .take(7)
        .map(|w| format!("{} ", weekday_cell(w)))
        .join("")
}

/// A cell like " 1" or "31".
fn day_cell(date: NaiveDate) -> String {
    let cell = format!("{:>2}", date.day());
    match date.weekday() {
        Weekday::Sat | Weekday::Sun => cell.red().to_string(),
        _ => cell,
    }
}

/// A line like " 8  9 10 11 12 13 14 ".
/// Current month must be provided to determine which days to show.
fn day_line(date: NaiveDate, start: Weekday, cur_month: u32) -> String {
    date.week(start)
        .first_day()
        .iter_days()
        .take(7)
        .map(|d| {
            if d.month() == cur_month {
                format!("{} ", day_cell(d))
            } else {
                "   ".to_string()
            }
        })
        .join("")
}

/// Multiple lines for days in a month.
fn day_lines(date: NaiveDate, start: Weekday) -> impl Iterator<Item = String> {
    date.with_day(1)
        .unwrap()
        .iter_weeks()
        .take(DAY_ROWS)
        .map(move |d| day_line(d, start, date.month()))
}

/// A full month calendar.
fn calendar(date: NaiveDate, start: Weekday) -> impl Iterator<Item = String> {
    std::iter::once(month_year_line(date))
        .chain(std::iter::once(weekday_line(start)))
        .chain(day_lines(date, start))
}

pub struct Calendar {
    /// the queried date
    query: NaiveDate,

    /// how many months to display
    nmon: u32,

    /// whether to span the queried date
    span: bool,

    /// the first weekday
    fday: Weekday,

    /// horizontal capacity of months
    ncol: usize,
}

impl Calendar {
    pub fn new(
        year: i32,
        month: u32,
        day: u32,
        nmon: u32,
        span: bool,
        fday: u8,
        ncol: usize,
    ) -> Option<Self> {
        Some(Self {
            query: NaiveDate::from_ymd_opt(year, month, day)?,
            nmon,
            span,
            fday: Weekday::from_u8(fday)?.pred(),
            ncol,
        })
    }

    fn iter_month(&self) -> impl Iterator<Item = NaiveDate> {
        itertools::iterate(
            if self.span {
                self.query - Months::new(self.nmon / 2)
            } else {
                self.query
            },
            |d| *d + Months::new(1),
        )
        .take(self.nmon as usize)
    }

    fn format(&self) -> String {
        self.iter_month()
            .map(|m| calendar(m, self.fday))
            .collect_vec()
            .chunks_mut(self.ncol)
            .flat_map(|vec_of_iters| {
                (0..MONTH_ROWS).map(|_| {
                    vec_of_iters
                        .iter_mut()
                        .map(|it| it.next().unwrap())
                        .join(" ")
                })
            })
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
    use regex::Regex;

    fn strip_color(s: &str) -> String {
        let re = Regex::new(r"\x1b\[\d+m").unwrap();
        re.replace_all(&s, "").to_string()
    }

    #[test]
    fn month_year_line_test() {
        let date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
        assert_eq!(month_year_line(date), "    January 2022     ");
        let date = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        assert_eq!(month_year_line(date), "    November 2022    ");
    }

    #[test]
    fn weekday_line_test() {
        let su = "\x1b[31mSu\x1b[0m Mo Tu We Th Fr \x1b[31mSa\x1b[0m ";
        assert_eq!(weekday_line(Weekday::Sun), su);
        let mo = "Mo Tu We Th Fr \x1b[31mSa\x1b[0m \x1b[31mSu\x1b[0m ";
        assert_eq!(weekday_line(Weekday::Mon), mo);
    }

    #[test]
    fn day_line_test() {
        let date = NaiveDate::from_ymd_opt(2022, 11, 1).unwrap();
        let cur_line = "       1  2  3  4 \x1b[31m 5\x1b[0m ";
        assert_eq!(day_line(date, Weekday::Sun, 11), cur_line);
        let prev_line = "\x1b[31m30\x1b[0m 31                ";
        assert_eq!(day_line(date, Weekday::Sun, 10), prev_line);
    }

    #[test]
    fn calendar_vec() {
        let date = NaiveDate::from_ymd_opt(2022, 11, 11).unwrap();
        let cal: Vec<_> = calendar(date, Weekday::Sun).collect();
        assert_eq!(
            cal,
            [
                "    November 2022    ",
                "\x1b[31mSu\x1b[0m Mo Tu We Th Fr \x1b[31mSa\x1b[0m ",
                "       1  2  3  4 \x1b[31m 5\x1b[0m ",
                "\x1b[31m 6\x1b[0m  7  8  9 10 11 \x1b[31m12\x1b[0m ",
                "\x1b[31m13\x1b[0m 14 15 16 17 18 \x1b[31m19\x1b[0m ",
                "\x1b[31m20\x1b[0m 21 22 23 24 25 \x1b[31m26\x1b[0m ",
                "\x1b[31m27\x1b[0m 28 29 30          ",
                "                     "
            ]
        );
    }

    #[test]
    fn draw_single_month() {
        let cal = Calendar::new(2022, 11, 1, 1, false, 0, 3).unwrap();
        assert_eq!(
            strip_color(&cal.format()),
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
        let cal = Calendar::new(2022, 11, 1, 2, false, 0, 3).unwrap();
        assert_eq!(
            strip_color(&cal.format()),
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
        let cal = Calendar::new(2022, 1, 1, 12, false, 0, 3).unwrap();
        assert_eq!(
            strip_color(&cal.format()),
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
