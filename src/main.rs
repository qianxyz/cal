use carender::{Calendar, MONTH_WIDTH};

use chrono::{Datelike, Local};
use clap::Parser;

const MAX_NCOL: usize = 3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Show only a single month (default)
    #[arg(group = "nmon", short = '1', long = "one")]
    nmon_1: bool,

    /// Show three months spanning the date
    #[arg(group = "nmon", short = '3', long = "three")]
    nmon_3: bool,

    /// Show the whole year
    #[arg(group = "nmon", short = 'y', long = "year")]
    nmon_y: bool,

    /// Show NUM months starting with date's month
    #[arg(group = "nmon", short = 'n', long = "months", value_name = "NUM")]
    nmon_n: Option<u32>,

    /// Span the date when displaying multiple months
    #[arg(short = 'S', long, requires = "nmon_n")]
    span: bool,

    /// Sunday as first day of week (default)
    #[arg(group = "fday", short = 's', long = "sunday")]
    fday_s: bool,

    /// Monday as first day of week
    #[arg(group = "fday", short = 'm', long = "monday")]
    fday_m: bool,

    /// Set first day of week (Sunday = 0, Monday = 1, ...)
    #[arg(group = "fday", short = 'f', long = "first", value_name = "0-6")]
    fday_n: Option<u8>,

    /// Format calendar into NUM columns of months
    #[arg(short = 'c', long = "column", value_name = "NUM")]
    ncol: Option<usize>,

    /// defaults to current year
    year: Option<i32>,

    /// defaults to current month
    month: Option<u32>,

    /// defaults to current day
    day: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    let now = Local::now();
    let year = cli.year.unwrap_or_else(|| now.year());
    let m = cli.month.unwrap_or_else(|| now.month());
    let day = cli.day.unwrap_or(1);

    let (month, nmon, span) = if cli.nmon_1 {
        (m, 1, false)
    } else if cli.nmon_3 {
        (m, 3, true)
    } else if cli.nmon_y {
        (1, 12, false)
    } else if let Some(n) = cli.nmon_n {
        (m, n.max(1), cli.span)
    } else if cli.year.is_some() && cli.month.is_none() {
        // special case: `cal YEAR` should print whole year calendar
        (1, 12, false)
    } else {
        (m, 1, false)
    };

    let fday = match (cli.fday_s, cli.fday_m, cli.fday_n) {
        (_, true, _) => 1,
        (_, _, Some(n)) => n,
        _ => 0,
    };

    let ncol = cli
        .ncol
        .unwrap_or(match termsize::get() {
            Some(size) => (size.cols as usize / (MONTH_WIDTH + 1)).min(MAX_NCOL),
            None => MAX_NCOL,
        })
        .max(1);

    let cal = Calendar::new(year, month, day, nmon, span, fday, ncol).unwrap();

    println!("{}", cal);
}
