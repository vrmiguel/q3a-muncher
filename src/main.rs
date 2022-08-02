mod cause_of_death;
mod error;
mod extra_checked_ops;
mod instance_counter;
mod parser;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub use cause_of_death::{CauseOfDeath, CAUSES_OF_DEATH};
pub use error::{Error, Result};
use parser::LogParser;

fn run() -> Result<()> {
    let path =
        std::env::args_os().nth(1).ok_or(Error::MissingFile)?;

    let mut reader = ReallocBufReader::from(path)?;
    let mut parser = LogParser::new();

    while let Some(line) = reader.read_line()? {
        parser.parse_line(line)?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}

struct ReallocBufReader {
    reader: BufReader<File>,
    buffer: String,
}

impl ReallocBufReader {
    pub fn from<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let buffer = String::with_capacity(1024);

        Ok(Self { reader, buffer })
    }

    pub fn read_line(&mut self) -> Result<Option<&str>> {
        self.buffer.clear();

        let bytes_read =
            self.reader.read_line(&mut self.buffer)?;

        Ok((bytes_read != 0).then(|| self.buffer.as_str()))
    }
}
