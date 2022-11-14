use chrono::{NaiveDate, Datelike, Month};
use num_traits::cast::FromPrimitive;

const MONTH_WIDTH: usize = 21;

fn calendar_lines(date: NaiveDate) -> Vec<String> {
    let mut cal = Vec::new();

    // header containing month and year
    let month = Month::from_u32(date.month()).unwrap();
    let header = format!("{} {}", month.name(), date.year());
    let padded_header = format!("{:^1$}", header, MONTH_WIDTH);
    cal.push(padded_header);

    cal.push("Su Mo Tu We Th Fr Sa ".to_string());



    cal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines() {
        let date = NaiveDate::from_ymd_opt(2022, 11, 15).unwrap();
        assert_eq!(calendar_lines(date), vec![""]);
    }
}
