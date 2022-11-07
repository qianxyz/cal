type Year = u32;
type Month = u8; // 1 = January, ...
type Weekday = u8; // 0 = Sunday, ...

fn is_leap(year: Year) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else {
        year % 4 == 0
    }
}

fn month_length(year: Year, month: Month) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}

fn day_of_week(year: Year, month: Month, dom: u8) -> Weekday {
    let a: u32 = (14 - month as u32) / 12;
    let y: u32 = year - a;
    let m: u32 = month as u32 + 12 * a - 2;

    ((dom as u32 + y + y / 4 - y / 100 + y / 400 + 31 * m / 12) % 7) as u8
}

fn month_year_header(year: Year, month: Month) -> String {
    let smon = match month {
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

    let header = format!("{} {}", smon, year);
    format!("{:^21}", header)
}

fn calendar_month(year: Year, month: Month) -> Vec<String> {
    let mut cal = Vec::new();
    cal.push(month_year_header(year, month));
    cal.push(String::from("Su Mo Tu We Th Fr Sa "));

    let month_length = month_length(year, month);
    let weekday_of_first = day_of_week(year, month, 1);
    for nline in 1..=6 {
        let mut line = String::new();
        let start = 1 - weekday_of_first as i8 + 7 * (nline - 1);
        for d in start..start + 7 {
            let s = if d >= 1 && d <= month_length as i8 {
                format!("{:>2} ", d)
            } else {
                String::from("   ")
            };
            line.push_str(&s);
        }
        cal.push(line);
    }

    cal
}

fn span_months(
    year: Year,
    month: Month,
    nmons: u8,
) -> impl Iterator<Item = (Year, Month)> {
    (0..nmons).map(move |n| {
        (year + (month + n - 1) as u32 / 12, (month + n - 1) % 12 + 1)
    })
}

fn draw_calendar(year: Year, month: Month, nmons: u8) -> String {
    span_months(year, month, nmons)
        .map(|(y, m)| calendar_month(y, m))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|cs| {
            (0..cs[0].len())
                .map(|i| {
                    cs.iter()
                        .map(|c| c[i].to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_leap() {
        assert!(is_leap(2000));
        assert!(is_leap(2020));
        assert!(!is_leap(2021));
        assert!(!is_leap(2100));
    }

    #[test]
    fn test_month_length() {
        assert_eq!(month_length(2020, 1), 31);
        assert_eq!(month_length(2020, 2), 29);
        assert_eq!(month_length(2020, 4), 30);
        assert_eq!(month_length(2021, 2), 28);
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(day_of_week(2022, 11, 6), 0);
        assert_eq!(day_of_week(2022, 11, 7), 1);
        assert_eq!(day_of_week(2022, 11, 8), 2);
        assert_eq!(day_of_week(2022, 11, 13), 0);
    }

    #[test]
    fn test_header() {
        assert_eq!(month_year_header(2022, 1), "    January 2022     ");
        assert_eq!(month_year_header(2022, 2), "    February 2022    ");
        assert_eq!(month_year_header(2022, 3), "     March 2022      ");
    }

    #[test]
    fn test_calendar_month() {
        assert_eq!(
            calendar_month(2022, 11),
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
    fn test_span_months() {
        let yms: Vec<_> = span_months(2022, 12, 14).collect();
        assert_eq!(
            yms,
            [
                (2022, 12),
                (2023, 1),
                (2023, 2),
                (2023, 3),
                (2023, 4),
                (2023, 5),
                (2023, 6),
                (2023, 7),
                (2023, 8),
                (2023, 9),
                (2023, 10),
                (2023, 11),
                (2023, 12),
                (2024, 1),
            ]
        );
    }

    #[test]
    fn draw_single_month() {
        assert_eq!(
            draw_calendar(2022, 11, 1),
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
    fn draw_three_months() {
        assert_eq!(
            draw_calendar(2022, 10, 3),
            "\
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

    #[test]
    fn draw_year() {
        assert_eq!(
            draw_calendar(2022, 1, 12),
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
