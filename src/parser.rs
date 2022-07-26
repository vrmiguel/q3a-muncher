#![allow(unused)]
mod combinator;
mod header;

use std::{collections::HashMap, rc::Rc};

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
    combinator::{
        parse_kill, parse_kill_message, parse_kill_metadata,
    },
    header::{parse_header, Header},
};
use crate::{
    extra_checked_ops::ExtraCheckedOps,
    instance_counter::InstanceCounter, CauseOfDeath, Error,
    Result, CAUSES_OF_DEATH,
};

const WORLD: &str = "<world>";

pub type CauseOfDeathCounter =
    InstanceCounter<CauseOfDeath, CAUSES_OF_DEATH>;

pub struct LogParser {
    cause_of_death_counter: CauseOfDeathCounter,
    players: Vec<Rc<str>>,
    scores: HashMap<Rc<str>, i32>,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            cause_of_death_counter: InstanceCounter::new(),
            players: vec![],
            scores: HashMap::new(),
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<()> {
        let (rest, action) =
            parse_header(input).map_err(Self::convert_error)?;

        match action {
            Header::InitGame => {}
            Header::Kill => {
                self.handle_kill(rest)?;
            }
            Header::ShutdownGame => {}
            _ => {
                // Not relevant for this application
            }
        }

        Ok(())
    }

    /// Insert the given username into the parser's
    /// player buffer or return it if already inserted
    fn get_or_insert_player(
        &mut self,
        username: &str,
    ) -> Rc<str> {
        match self
            .players
            .iter()
            .find(|player| player.as_ref() == username)
        {
            Some(player) => player.clone(),
            None => {
                let ref_counted: Rc<str> = Rc::from(username);
                self.players.push(ref_counted.clone());
                ref_counted
            }
        }
    }

    fn convert_error(
        error: nom::Err<nom::error::Error<&str>>,
    ) -> Error {
        // TODO
        Error::ParsingError
    }

    fn handle_kill<'a>(
        &mut self,
        input: &'a str,
    ) -> Result<&'a str> {
        let (rest, message) =
            parse_kill(input).map_err(Self::convert_error)?;

        self.cause_of_death_counter
            .add(message.cause_of_death)?;

        let victim = self.get_or_insert_player(message.victim);

        if message.attacker == WORLD {
            // Victim must get discounted one point
            self.scores
                .entry(victim)
                .or_default()
                .checked_decrement();
        } else {
            let attacker =
                self.get_or_insert_player(message.attacker);

            self.scores
                .entry(victim)
                .or_default()
                .checked_increment();
        }

        Ok(rest)
    }
}

#[cfg(test)]
mod tests {
    use super::LogParser;

    #[test]
    fn parser_saves_players_correctly() {
        let mut parser = LogParser::new();

        parser.parse(" 21:42 Kill: 1022 2 22: <world> killed Isgalamido by MOD_TRIGGER_HURT").unwrap();
    }
}
