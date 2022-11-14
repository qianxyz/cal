use std::cmp;

use crate::error::CalResult;
use crate::format::CalFormat;
use crate::range::{CalRange, MonthOfYear};
use crate::wrapper::Weekday;

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

    /// format calendar into NUM columns of months
    #[arg(short, long, value_name = "NUM")]
    column: Option<usize>,

    /// defaults to current year
    year: Option<u32>,

    /// defaults to current month
    month: Option<u8>,
}

fn default_column() -> usize {
    const MAX_COL: usize = 3;
    const DEFAULT_TERM_WIDTH: u16 = 80;

    match termsize::get() {
        Some(size) if size.cols < DEFAULT_TERM_WIDTH => {
            cmp::max(size.cols as usize / MonthOfYear::MONTH_WIDTH, 1)
        }
        _ => MAX_COL,
    }
}

pub fn parse_cli() -> CalResult<CalFormat> {
    let cli = Cli::parse();

    let now = Local::now();

    let year = cli.year.unwrap_or(now.year() as u32);
    let month = cli.month.unwrap_or(now.month() as u8);

    // parse a month range
    let range = if cli.len_1 {
        CalRange::new(year, month, 1, false)?
    } else if cli.len_3 {
        CalRange::new(year, month, 3, true)?
    } else if cli.len_y {
        CalRange::new(year, 1, 12, false)?
    } else if let Some(n) = cli.len_n {
        CalRange::new(year, month, n, cli.span)?
    } else {
        // special case: `cal YEAR` should print whole year calendar
        if cli.year.is_some() && cli.month.is_none() {
            CalRange::new(year, 1, 12, false)?
        } else {
            CalRange::new(year, month, 1, false)?
        }
    };

    let fday: Weekday = match (cli.fday_s, cli.fday_m, cli.fday_n) {
        (_, true, _) => Weekday::Monday,
        (_, _, Some(n)) => Weekday::try_from(n)?,
        _ => Weekday::Sunday,
    };

    let column = match cli.column {
        Some(c) => cmp::max(c, 1),
        None => default_column(),
    };

    Ok(CalFormat::new(range, fday, column))
}
