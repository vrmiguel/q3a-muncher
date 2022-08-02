mod combinator;
mod display;
mod header;

use std::{collections::HashMap, rc::Rc};

use nom::{Finish, IResult};

use self::{
    combinator::parse_kill,
    header::{parse_header, Header},
};
use crate::{
    ensure, extra_checked_ops::ExtraCheckedOps,
    instance_counter::InstanceCounter, CauseOfDeath, Error,
    Result, CAUSES_OF_DEATH,
};

const WORLD: &str = "<world>";

pub type CauseOfDeathCounter =
    InstanceCounter<CauseOfDeath, CAUSES_OF_DEATH>;

/// A parser for Quake 3 Arena logs
pub struct LogParser {
    /// The index of the current game.
    game_idx: u8,
    /// How many kills happened during this game,
    /// including the ones caused by `<world>`.
    total_kills: u32,
    /// Totals up how many deaths were caused by each
    /// cause of death.
    cause_of_death_counter: CauseOfDeathCounter,
    /// Maps each player to his score.
    scores: HashMap<Rc<str>, i32>,
}

impl LogParser {
    /// Build a new, empty `LogpParser`
    pub fn new() -> Self {
        Self {
            cause_of_death_counter: InstanceCounter::new(),
            scores: HashMap::new(),
            game_idx: 0,
            total_kills: 0,
        }
    }

    /// Parses a single line of a Quake 3 Arena log.
    ///
    /// If a game has ended, this function will print
    /// a report of the game to stdout.
    pub fn parse_line(&mut self, input: &str) -> Result<()> {
        let (rest, action) =
            parse_header(input).map_err(Self::convert_error)?;

        match action {
            Header::Kill => {
                self.handle_kill(rest)?;
            }
            Header::ShutdownGame => self.handle_shutdown()?,
            _ => {
                // Not relevant for this application
            }
        }

        Ok(())
    }

    /// Insert the given username into the parser's
    /// player buffer or return it if already inserted.
    fn intern_username(&mut self, username: &str) -> Rc<str> {
        self.scores
            .get_key_value(username)
            .map(|(cached_username, _)| cached_username.clone())
            .unwrap_or_else(|| Rc::from(username))
    }

    /// Converts a `nom` Error into a `crate::Error`
    fn convert_error(
        error: nom::Err<nom::error::Error<&str>>,
    ) -> Error {
        let result: IResult<_, &str> = Err(error.to_owned());
        let result = result.finish();

        // This result is guaranteed to be an error,
        // so unwrap_err will never fail
        Error::ParsingError(result.unwrap_err())
    }

    fn handle_shutdown(&mut self) -> Result<()> {
        println!("{self}");

        self.clear();
        Ok(())
    }

    fn clear(&mut self) {
        // Get the game counter ready for the next game ..
        self.game_idx += 1;

        // .. and then reset all the rest
        self.scores.clear();
        self.total_kills = 0;
        self.cause_of_death_counter = InstanceCounter::new();
    }

    fn handle_kill(&mut self, input: &str) -> Result<()> {
        let (rest, message) =
            parse_kill(input).map_err(Self::convert_error)?;

        ensure!(
            rest.trim().is_empty(),
            "Line contained unexpected input"
        );

        self.cause_of_death_counter
            .add(message.cause_of_death)?;

        let victim = self.intern_username(message.victim);

        if message.attacker == WORLD {
            // Victim must get discounted one point
            self.scores
                .entry(victim)
                .or_default()
                .checked_decrement()?;
        } else {
            let attacker =
                self.intern_username(message.attacker);

            self.scores
                .entry(attacker)
                .or_default()
                .checked_increment()?;
        }

        self.total_kills.checked_increment()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::LogParser;
    use crate::CauseOfDeath;

    #[test]
    fn parser_saves_players_correctly() {
        let mut parser = LogParser::new();
        let snek: Rc<str> = Rc::from("snek");
        let crab: Rc<str> = Rc::from("crab");
        let gopher: Rc<str> = Rc::from("gopher");

        parser.parse_line(" 21:42 Kill: 1022 2 22: crab killed gopher by MOD_ROCKET").unwrap();
        parser.parse_line(" 21:42 Kill: 1022 2 22: crab killed gopher by MOD_ROCKET").unwrap();
        parser.parse_line(" 21:43 Kill: 1022 2 22: crab killed snek by MOD_ROCKET").unwrap();
        parser.parse_line(" 21:43 Kill: 1022 2 22: <world> killed gopher by MOD_LAVA").unwrap();

        assert_eq!(
            parser
                .cause_of_death_counter
                .get(CauseOfDeath::Rocket)
                .unwrap(),
            3
        );

        // assert_eq!(
        //     HashSet::from_iter(parser.players.into_iter()),
        //     HashSet::from([
        //         snek.clone(),
        //         crab.clone(),
        //         gopher.clone()
        //     ])
        // );

        assert_eq!(*parser.scores.get(&crab).unwrap(), 3);
        assert_eq!(*parser.scores.get(&gopher).unwrap(), -1);

        // `snek` didn't score so it did not get included in the
        // map. This is fixed in `LogParser::handle_shutdown`
        // before printing the report.
        assert_eq!(parser.scores.get(&snek), None);
    }

    #[test]
    fn parser_saves_players_correctly_when_killed_by_world() {
        let mut parser = LogParser::new();
        let player: Rc<str> = Rc::from("xXplayerXx");

        parser.parse_line(" 21:42 Kill: 1022 2 22: <world> killed xXplayerXx by MOD_TRIGGER_HURT").unwrap();

        assert_eq!(
            parser
                .cause_of_death_counter
                .get(CauseOfDeath::TriggerHurt)
                .unwrap(),
            1
        );
        // assert_eq!(parser.players, &[player.clone()]);
        assert_eq!(*parser.scores.get(&player).unwrap(), -1);
    }
}
