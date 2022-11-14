mod cli;
mod error;
mod format;
mod range;
mod wrapper;

fn main() -> error::CalResult<()> {
    let cal = cli::parse_cli()?;
    println!("{}", cal);

    Ok(())
}
