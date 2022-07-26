#![allow(unused)]
mod combinator;
mod header;

use nom::{
    branch::alt,
    bytes::complete::{
        tag, take, take_while, take_while1, take_while_m_n,
    },
    character::complete::char,
    combinator::value,
    multi::separated_list0,
    number::complete::double,
    sequence::{
        delimited, preceded, separated_pair, terminated,
    },
    Finish, IResult, Parser,
};

use self::{
    combinator::parse_kill_metadata,
    header::{parse_header, Header},
};
use crate::{
    instance_counter::InstanceCounter, CauseOfDeath, Error,
    Result, CAUSES_OF_DEATH,
};

pub type CauseOfDeathCounter =
    InstanceCounter<CauseOfDeath, CAUSES_OF_DEATH>;

pub struct LogParser {
    cause_of_death_counter: CauseOfDeathCounter,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            cause_of_death_counter: InstanceCounter::new(),
        }
    }

    pub fn parse(&self, input: &str) -> Result<()> {
        let (rest, action) =
            parse_header(input).map_err(Self::convert_error)?;

        match action {
            Header::InitGame => {}
            Header::Kill => {
                self.handle_kill(rest)
                    .map_err(Self::convert_error)?;
            }
            Header::ShutdownGame => {}
            _ => {
                // Not relevant for this application
            }
        }

        Ok(())
    }

    fn convert_error(
        error: nom::Err<nom::error::Error<&str>>,
    ) -> Error {
        // TODO
        Error::ParsingError
    }

    fn handle_kill<'a>(
        &self,
        input: &'a str,
    ) -> IResult<&'a str, &'a str> {
        let (rest, _) = parse_kill_metadata(input)?;

        // Ok(())
        todo!()
    }
}

#[cfg(test)]
mod tests {
    // use super::parse_timestamp;
    use crate::parser::{parse_header, Header};
}
