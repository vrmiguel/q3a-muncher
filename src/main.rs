mod cause_of_death;
mod error;
mod extra_checked_ops;
mod instance_counter;
mod parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub use cause_of_death::{CauseOfDeath, CAUSES_OF_DEATH};
pub use error::{Error, Result};
use parser::LogParser;

fn run() -> Result<()> {
    let path =
        std::env::args_os().nth(1).ok_or(Error::MissingFile)?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut parser = LogParser::new();

    for line in reader.lines() {
        let line = line?;
        parser.parse(&line)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}
