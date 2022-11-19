use carender::Calendar;

use chrono::{Datelike, Local};
use clap::Parser;

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

    /// Defaults to current year
    year: Option<i32>,

    /// Defaults to current month
    month: Option<u32>,

    /// Defaults to current day
    day: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    let now = Local::now();
    let y = cli.year.unwrap_or_else(|| now.year());
    let m = cli.month.unwrap_or_else(|| now.month());
    let d = cli.day.unwrap_or(1);

    let (nmon, span, year) = if cli.nmon_1 {
        (1, false, false)
    } else if cli.nmon_3 {
        (3, true, false)
    } else if cli.nmon_y {
        (12, false, true)
    } else if let Some(n) = cli.nmon_n {
        (n.max(1), cli.span, false)
    } else if cli.year.is_some() && cli.month.is_none() {
        // special case: `cal YEAR` should print whole year calendar
        (12, false, true)
    } else {
        (1, false, false)
    };

    let fday = match (cli.fday_s, cli.fday_m, cli.fday_n) {
        (_, true, _) => 1,
        (_, _, Some(n)) => n,
        _ => 0,
    };

    let ncol = cli.ncol;

    let hlight = if cli.day.is_some() {
        (y, m, d)
    } else {
        (now.year(), now.month(), now.day())
    };

    let cal = Calendar::new((y, m, d), nmon, span, year, fday, ncol, hlight).unwrap();

    println!("{}", cal);
}
