use std::{
    collections::HashMap,
    fmt::{Display, Write},
    rc::Rc,
};

use super::LogParser;
use crate::{
    parser::CauseOfDeathCounter, CauseOfDeath, CAUSES_OF_DEATH,
};

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

            writeln!(f, "\t}},")
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
                // if incidence == 0 {
                //     continue;
                // }

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
        writeln!(f, "\t\"total_kills\": {},", self.total_kills)?;
        write_players(f, &self.players)?;
        write_score(f, &self.scores)?;
        write_means_of_death(f, &self.cause_of_death_counter)?;

        f.write_char('}')
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader, Cursor};

    use crate::parser::LogParser;

    const INPUT: &str = r#"
    0:16 Kill: 6 2 7: Zeh killed Isgalamido by MOD_ROCKET_SPLASH
    0:16 Kill: 2 7 6: Isgalamido killed Mal by MOD_ROCKET
    0:17 Item: 4 weapon_rocketlauncher
    0:18 Item: 5 item_armor_shard
    0:18 Item: 5 item_armor_shard
    0:18 Item: 5 item_armor_shard
    0:18 Item: 5 item_armor_combat
    0:21 Item: 6 weapon_rocketlauncher
    0:23 Item: 2 weapon_rocketlauncher
    0:26 Kill: 5 3 7: Assasinu Credi killed Oootsimo by MOD_ROCKET_SPLASH
    0:27 Kill: 1022 4 19: <world> killed Dono da Bola by MOD_FALLING
    0:28 Kill: 1022 7 22: <world> killed Mal by MOD_TRIGGER_HURT
    0:31 Item: 7 weapon_rocketlauncher
    0:31 Kill: 5 6 7: Assasinu Credi killed Zeh by MOD_ROCKET_SPLASH
    0:32 Item: 5 weapon_rocketlauncher"#;

    #[test]
    fn display_impl_works() {
        let mut parser = LogParser::new();
        let reader = BufReader::new(Cursor::new(INPUT));

        for line in reader.lines().skip(1) {
            let line = line.unwrap();
            parser.parse_line(&line).unwrap();
        }

        let gotten = format!("{{{parser}}}");
        let gotten: serde_json::Value =
            serde_json::from_str(&gotten).unwrap();

        let expected = serde_json::json!({
            "game0": {
                "total_kills": 6,
                "players": ["Isgalamido", "Zeh", "Mal", "Oootsimo", "Assasinu Credi", "Dono da Bola"],
                "kills": {
                        "Mal": -1,
                        "Isgalamido": 1,
                        "Dono da Bola": -1,
                        "Zeh": 1,
                        "Assasinu Credi": 2
                },
                "kills_by_means": {
                        "MOD_SHOTGUN": 0,
                        "MOD_GAUNTLET": 0,
                        "MOD_MACHINEGUN": 0,
                        "MOD_GRENADE": 0,
                        "MOD_GRENADE_SPLASH": 0,
                        "MOD_ROCKET": 1,
                        "MOD_ROCKET_SPLASH": 3,
                        "MOD_PLASMA": 0,
                        "MOD_PLASMA_SPLASH": 0,
                        "MOD_RAILGUN": 0,
                        "MOD_LIGHTNING": 0,
                        "MOD_BFG": 0,
                        "MOD_BFG_SPLASH": 0,
                        "MOD_WATER": 0,
                        "MOD_SLIME": 0,
                        "MOD_LAVA": 0,
                        "MOD_CRUSH": 0,
                        "MOD_TELEFRAG": 0,
                        "MOD_FALLING": 1,
                        "MOD_SUICIDE": 0,
                        "MOD_TARGET_LASER": 0,
                        "MOD_TRIGGER_HURT": 1,
                        "MOD_NAIL": 0,
                        "MOD_CHAINGUN": 0,
                        "MOD_PROXIMITY_MINE": 0,
                        "MOD_KAMIKAZE": 0,
                        "MOD_JUICED": 0,
                        "MOD_GRAPPLE": 0,
                        "MOD_UNKNOWN": 0
                }
            }
        });

        assert_eq!(gotten, expected);
    }
}
