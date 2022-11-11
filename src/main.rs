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

    /// span the date when displaying multiple months
    #[arg(short = 'S', long)]
    span: bool,

    /// Sunday as first day of week (default)
    #[arg(group = "fday", short = 's', long = "sunday")]
    fday_s: bool,

    /// Monday as first day of week
    #[arg(group = "fday", short = 'm', long = "monday")]
    fday_m: bool,

    /// set first day of week (Sunday = 0, Monday = 1, ...)
    #[arg(group = "fday", short = 'f', long = "first", value_name = "0-6")]
    fday_n: Option<u8>,

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

    let (origin, len) = if cli.len_1 {
        (YearMonth::new(year, month), 1)
    } else if cli.len_3 {
        (YearMonth::new(year, month), 3)
    } else if cli.len_y {
        (YearMonth::new(year, 1), 12)
    } else if cli.len_12 {
        /* special case: `cal -Y YEAR` should print whole year calendar */
        if cli.year.is_some() && cli.month.is_none() {
            (YearMonth::new(year, 1), 12)
        } else {
            (YearMonth::new(year, month), 12)
        }
    } else if let Some(n) = cli.len_n {
        (YearMonth::new(year, month), std::cmp::max(n, 1))
    } else {
        /* special case: `cal YEAR` should print whole year calendar */
        if cli.year.is_some() && cli.month.is_none() {
            (YearMonth::new(year, 1), 12)
        } else {
            (YearMonth::new(year, month), 1)
        }
    };

    let span = cli.len_3 || cli.span;

    let fday = match (cli.fday_s, cli.fday_m, cli.fday_n) {
        (_, true, _) => 1,
        (_, _, Some(n)) => n,
        _ => 0,
    };

    let cal = Calendar::new(origin, len, span, fday);

    println!("{}", cal);
}
