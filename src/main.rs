use carender::*;

use chrono::{Datelike, Local};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// show only a single month (default)
    #[arg(group = "len", short = '1', long = "one")]
    len_1: bool,

    /// show three months spanning the date
    #[arg(group = "len", short = '3', long = "three")]
    len_3: bool,

    /// show the whole year
    #[arg(group = "len", short = 'y', long = "year")]
    len_y: bool,

    /// show the next twelve months
    #[arg(group = "len", short = 'Y', long = "twelve")]
    len_12: bool,

    /// show NUM months starting with date's month
    #[arg(group = "len", short = 'n', long = "months", value_name = "NUM")]
    len_n: Option<usize>,

    /// defaults to current year
    year: Option<Year>,

    /// defaults to current month
    month: Option<Month>,
}

fn main() {
    let cli = Cli::parse();

    let now = Local::now();
    let year = cli.year.unwrap_or(now.year() as Year);
    let month = cli.month.unwrap_or(now.month() as Month);

    let (start, len) = if cli.len_3 {
        (
            if month == 1 {
                YearMonth::new(year - 1, 12)
            } else {
                YearMonth::new(year, month - 1)
            },
            3,
        )
    } else if cli.len_y {
        (YearMonth::new(year, 1), 12)
    } else if cli.len_12 {
        (YearMonth::new(year, month), 12)
    } else if let Some(n) = cli.len_n {
        (YearMonth::new(year, month), n)
    } else {
        (YearMonth::new(year, month), 1)
    };

    let cal = CalRange::new(start, len);

    println!("{}", cal);
}
