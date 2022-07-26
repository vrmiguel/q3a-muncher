mod combinator;
mod header;

use std::{
    collections::HashMap,
    fmt::{Display, Write},
    ops::Not,
    rc::Rc,
};

use nom::{Finish, IResult};

use self::{
    combinator::parse_kill,
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
    /// The index of the current game.
    game_idx: u8,
    /// How many kills happened during this game,
    /// including the ones caused by `<world>`.
    total_kills: u32,
    /// Totals up how many deaths were caused by each
    /// cause of death.
    cause_of_death_counter: CauseOfDeathCounter,
    /// The players in a game.
    players: Vec<Rc<str>>,
    /// Maps each player to his score.
    scores: HashMap<Rc<str>, i32>,
}

impl LogParser {
    pub fn new() -> Self {
        Self {
            cause_of_death_counter: InstanceCounter::new(),
            players: vec![],
            scores: HashMap::new(),
            game_idx: 0,
            total_kills: 0,
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<()> {
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
        let result: IResult<_, &str> = Err(error.to_owned());
        let result = result.finish();

        // This result is guaranteed to be an error,
        // so unwrap_err will never fail
        Error::ParsingError(result.unwrap_err())
    }

    fn handle_shutdown(&mut self) -> Result<()> {
        if self.players.len() != self.scores.len() {
            // Players that didn't score haven't been inserted
            // into the `scores` map yet, so the function
            // below fixes that
            self.fill_out_scores();
        }

        println!("{self}");

        self.clear();
        Ok(())
    }

    fn fill_out_scores(&mut self) {
        for player in &self.players {
            if self.scores.contains_key(player).not() {
                self.scores.entry(player.clone()).or_default();
            }
        }
    }

    fn clear(&mut self) {
        self.players.clear();
        self.scores.clear();
        self.game_idx += 1;
        self.total_kills = 0;
        self.cause_of_death_counter = InstanceCounter::new();
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
                .checked_decrement()?;
        } else {
            let attacker =
                self.get_or_insert_player(message.attacker);

            self.scores
                .entry(attacker)
                .or_default()
                .checked_increment()?;
        }

        self.total_kills.checked_increment()?;

        Ok(rest)
    }
}

/// Mini built-in JSON formatter :P
impl Display for LogParser {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        fn write_players(
            f: &mut std::fmt::Formatter<'_>,
            slice: &[Rc<str>],
        ) -> std::fmt::Result {
            write!(f, "\t\"players\": [")?;

            if let Some((last, elems)) = slice.split_last() {
                for elem in elems {
                    write!(f, "\"{elem}\", ")?;
                }
                write!(f, "\"{last}\"")?;
            }

            f.write_str("],\n")
        }

        fn write_score(
            f: &mut std::fmt::Formatter<'_>,
            scores: &HashMap<Rc<str>, i32>,
        ) -> std::fmt::Result {
            writeln!(f, "\t\"kills\": {{")?;

            let length = scores.len();

            for (idx, (player, &score)) in
                scores.iter().enumerate()
            {
                write!(f, "\t\t\"{player}\": {score}")?;

                if idx + 1 != length {
                    writeln!(f, ",")?;
                } else {
                    writeln!(f)?;
                }
            }

            writeln!(f, "\t}}")
        }

        fn write_means_of_death(
            f: &mut std::fmt::Formatter<'_>,
            counter: &CauseOfDeathCounter,
        ) -> std::fmt::Result {
            writeln!(f, "\t\"kills_by_means\": {{")?;

            let length = CAUSES_OF_DEATH;

            for idx in 0..CAUSES_OF_DEATH {
                // Should not fail: this same operation is done
                // during testing
                let cause_of_death =
                    CauseOfDeath::try_from(idx as u8).unwrap();
                let incidence =
                    counter.get(cause_of_death).unwrap_or(0);
                if incidence == 0 {
                    continue;
                }

                write!(
                    f,
                    "\t\t\"{cause_of_death}\": {incidence}"
                )?;

                if idx + 1 != length {
                    writeln!(f, ",")?;
                } else {
                    writeln!(f)?;
                }
            }

            writeln!(f, "\t}}")
        }

        writeln!(f, "\"game{}\": {{", self.game_idx)?;
        writeln!(f, "\t\"total_kills\": {}", self.total_kills)?;
        write_players(f, &self.players)?;
        write_score(f, &self.scores)?;
        write_means_of_death(f, &self.cause_of_death_counter)?;

        f.write_char('}')
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, rc::Rc};

    use super::LogParser;
    use crate::CauseOfDeath;

    #[test]
    fn parser_saves_players_correctly() {
        let mut parser = LogParser::new();
        let snek: Rc<str> = Rc::from("snek");
        let crab: Rc<str> = Rc::from("crab");
        let gopher: Rc<str> = Rc::from("gopher");

        parser.parse(" 21:42 Kill: 1022 2 22: crab killed gopher by MOD_ROCKET").unwrap();
        parser.parse(" 21:42 Kill: 1022 2 22: crab killed gopher by MOD_ROCKET").unwrap();
        parser.parse(" 21:43 Kill: 1022 2 22: crab killed snek by MOD_ROCKET").unwrap();
        parser.parse(" 21:43 Kill: 1022 2 22: <world> killed gopher by MOD_LAVA").unwrap();

        assert_eq!(
            parser
                .cause_of_death_counter
                .get(CauseOfDeath::Rocket)
                .unwrap(),
            3
        );

        assert_eq!(
            HashSet::from_iter(parser.players.into_iter()),
            HashSet::from([
                snek.clone(),
                crab.clone(),
                gopher.clone()
            ])
        );

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

        parser.parse(" 21:42 Kill: 1022 2 22: <world> killed xXplayerXx by MOD_TRIGGER_HURT").unwrap();

        assert_eq!(
            parser
                .cause_of_death_counter
                .get(CauseOfDeath::TriggerHurt)
                .unwrap(),
            1
        );
        assert_eq!(parser.players, &[player.clone()]);
        assert_eq!(*parser.scores.get(&player).unwrap(), -1);
    }
}
