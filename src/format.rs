use crate::range::{CalRange, MonthOfYear};

impl MonthOfYear {
    const MONTH_WIDTH: usize = 21;

    fn header(&self) -> String {
        let header = format!("{} {}", self.month(), self.year());
        format!("{:^1$}", header, Self::MONTH_WIDTH)
    }

    fn week_line(&self) -> String {
        todo!()
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
}
