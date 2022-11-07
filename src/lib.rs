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

fn calendar_line(year: Year, month: Month, line: u8) -> String {
    let mut ret = String::new();

    let month_length = month_length(year, month);
    let weekday_of_first = day_of_week(year, month, 1);
    let start = 1 - weekday_of_first as i8 + 7 * (line as i8 - 1);
    for d in start..start + 7 {
        let s = if d >= 1 && d <= month_length as i8 {
            format!("{:>2} ", d)
        } else {
            String::from("   ")
        };
        ret.push_str(&s);
    }

    ret
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
    fn test_calendar_line() {
        assert_eq!(
            calendar_line(2022, 11, 1),
            String::from("       1  2  3  4  5 ")
        );
        assert_eq!(
            calendar_line(2022, 11, 2),
            String::from(" 6  7  8  9 10 11 12 ")
        );
        assert_eq!(
            calendar_line(2022, 11, 3),
            String::from("13 14 15 16 17 18 19 ")
        );
        assert_eq!(
            calendar_line(2022, 11, 4),
            String::from("20 21 22 23 24 25 26 ")
        );
        assert_eq!(
            calendar_line(2022, 11, 5),
            String::from("27 28 29 30          ")
        );
        assert_eq!(
            calendar_line(2022, 11, 6),
            String::from("                     ")
        );
    }
}
