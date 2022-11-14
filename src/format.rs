use crate::range::{CalRange, MonthOfYear};
use crate::wrapper::Weekday;

impl Weekday {
    fn week_line(&self) -> String {
        (0..7)
            .map(|i| {
                format!(
                    "{} ",
                    Weekday::try_from((i + *self as u8) % 7)
                        .unwrap()
                        .to_string()
                )
            })
            .collect()
    }
}

impl MonthOfYear {
    const MONTH_WIDTH: usize = 21;
    const DAYS_HEIGHT: usize = 6;

    fn header(&self) -> String {
        let header = format!("{} {}", self.month(), self.year());
        format!("{:^1$}", header, Self::MONTH_WIDTH)
    }

    fn day_matrix(&self, fday: Weekday) -> impl Iterator<Item = String> {
        let start = 1 - (self.weekday_of_first() - fday);
        let month_length = self.num_of_days() as i8;
        (0..Self::DAYS_HEIGHT as i8).map(move |n| {
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
        cal.push(self.header());
        cal.push(fday.week_line());
        cal.extend(self.day_matrix(fday));

        cal
    }
}

struct CalFormat {
    /// range of months in calendar
    range: CalRange,

    /// first weekday of week
    fday: Weekday,

    /// num of months to display horizontally
    column: usize,
}

impl CalFormat {
    fn new(range: CalRange, fday: Weekday, column: usize) -> Self {
        Self {
            range,
            fday,
            column,
        }
    }

    fn format(&self) -> String {
        self.range
            .iter()
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

impl std::fmt::Display for CalFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moy(y: u32, m: u8) -> MonthOfYear {
        MonthOfYear::new(y, m).unwrap()
    }

    #[test]
    fn month_header() {
        assert_eq!(moy(2022, 1).header(), "    January 2022     ");
        assert_eq!(moy(2022, 2).header(), "    February 2022    ");
        assert_eq!(moy(2022, 3).header(), "     March 2022      ");
    }

    #[test]
    fn week_line() {
        assert_eq!(Weekday::Sunday.week_line(), "Su Mo Tu We Th Fr Sa ");
        assert_eq!(Weekday::Monday.week_line(), "Mo Tu We Th Fr Sa Su ");
        assert_eq!(Weekday::Saturday.week_line(), "Sa Su Mo Tu We Th Fr ");
    }

    #[test]
    fn calendar_vec() {
        assert_eq!(
            moy(2022, 11).calendar(Weekday::Sunday),
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
        assert_eq!(
            moy(2022, 11).calendar(Weekday::Monday),
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
            moy(2022, 11).calendar(Weekday::Wednesday),
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
    fn draw_single_month() {
        let range = CalRange::new(2022, 11, 1, false).unwrap();
        let cal = CalFormat::new(range, Weekday::Sunday, 3);
        assert_eq!(
            cal.format(),
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
        let range = CalRange::new(2022, 11, 2, false).unwrap();
        let cal = CalFormat::new(range, Weekday::Sunday, 3);
        assert_eq!(
            cal.format(),
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
        let range = CalRange::new(2022, 1, 12, false).unwrap();
        let cal = CalFormat::new(range, Weekday::Sunday, 3);
        assert_eq!(
            cal.format(),
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
