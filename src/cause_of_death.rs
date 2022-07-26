use std::{fmt::Display, str::FromStr};

/// How many causes of death there are.
pub const CAUSES_OF_DEATH: usize = 29;

#[cfg(test)]
use strum::EnumCount;

use crate::Error;
#[cfg(test)]
// Ensure `CAUSES_OF_DEATH` is correct
static_assertions::const_assert_eq!(
    CAUSES_OF_DEATH,
    CauseOfDeath::COUNT
);

/// Possible causes of death within a Q3A game
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(test, derive(strum::EnumIter, strum::EnumCount))]
#[repr(u8)]
pub enum CauseOfDeath {
    Shotgun,
    Gauntlet,
    Machinegun,
    Grenade,
    GrenadeSplash,
    Rocket,
    RocketSplash,
    Plasma,
    PlasmaSplash,
    Railgun,
    Lightning,
    Bfg,
    BfgSplash,
    Water,
    Slime,
    Lava,
    Crush,
    Telefrag,
    Falling,
    Suicide,
    TargetLaser,
    TriggerHurt,
    Nail,
    Chaingun,
    ProximityMine,
    Kamikaze,
    Juiced,
    Grapple,
    Unknown,
}

impl CauseOfDeath {
    /// Converts a cause of death into its
    /// expected Q3A formatting
    pub fn as_str(self) -> &'static str {
        match self {
            CauseOfDeath::Shotgun => "MOD_SHOTGUN",
            CauseOfDeath::Gauntlet => "MOD_GAUNTLET",
            CauseOfDeath::Machinegun => "MOD_MACHINEGUN",
            CauseOfDeath::Grenade => "MOD_GRENADE",
            CauseOfDeath::GrenadeSplash => "MOD_GRENADE_SPLASH",
            CauseOfDeath::Rocket => "MOD_ROCKET",
            CauseOfDeath::RocketSplash => "MOD_ROCKET_SPLASH",
            CauseOfDeath::Plasma => "MOD_PLASMA",
            CauseOfDeath::PlasmaSplash => "MOD_PLASMA_SPLASH",
            CauseOfDeath::Railgun => "MOD_RAILGUN",
            CauseOfDeath::Lightning => "MOD_LIGHTNING",
            CauseOfDeath::Bfg => "MOD_BFG",
            CauseOfDeath::BfgSplash => "MOD_BFG_SPLASH",
            CauseOfDeath::Water => "MOD_WATER",
            CauseOfDeath::Slime => "MOD_SLIME",
            CauseOfDeath::Lava => "MOD_LAVA",
            CauseOfDeath::Crush => "MOD_CRUSH",
            CauseOfDeath::Telefrag => "MOD_TELEFRAG",
            CauseOfDeath::Falling => "MOD_FALLING",
            CauseOfDeath::Suicide => "MOD_SUICIDE",
            CauseOfDeath::TargetLaser => "MOD_TARGET_LASER",
            CauseOfDeath::TriggerHurt => "MOD_TRIGGER_HURT",
            CauseOfDeath::Nail => "MOD_NAIL",
            CauseOfDeath::Chaingun => "MOD_CHAINGUN",
            CauseOfDeath::ProximityMine => "MOD_PROXIMITY_MINE",
            CauseOfDeath::Kamikaze => "MOD_KAMIKAZE",
            CauseOfDeath::Juiced => "MOD_JUICED",
            CauseOfDeath::Grapple => "MOD_GRAPPLE",
            CauseOfDeath::Unknown => "MOD_UNKNOWN",
        }
    }
}

impl From<CauseOfDeath> for u8 {
    fn from(cause: CauseOfDeath) -> Self {
        cause as Self
    }
}

impl TryFrom<u8> for CauseOfDeath {
    type Error = crate::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let cause = match value {
            0 => CauseOfDeath::Shotgun,
            1 => CauseOfDeath::Gauntlet,
            2 => CauseOfDeath::Machinegun,
            3 => CauseOfDeath::Grenade,
            4 => CauseOfDeath::GrenadeSplash,
            5 => CauseOfDeath::Rocket,
            6 => CauseOfDeath::RocketSplash,
            7 => CauseOfDeath::Plasma,
            8 => CauseOfDeath::PlasmaSplash,
            9 => CauseOfDeath::Railgun,
            10 => CauseOfDeath::Lightning,
            11 => CauseOfDeath::Bfg,
            12 => CauseOfDeath::BfgSplash,
            13 => CauseOfDeath::Water,
            14 => CauseOfDeath::Slime,
            15 => CauseOfDeath::Lava,
            16 => CauseOfDeath::Crush,
            17 => CauseOfDeath::Telefrag,
            18 => CauseOfDeath::Falling,
            19 => CauseOfDeath::Suicide,
            20 => CauseOfDeath::TargetLaser,
            21 => CauseOfDeath::TriggerHurt,
            22 => CauseOfDeath::Nail,
            23 => CauseOfDeath::Chaingun,
            24 => CauseOfDeath::ProximityMine,
            25 => CauseOfDeath::Kamikaze,
            26 => CauseOfDeath::Juiced,
            27 => CauseOfDeath::Grapple,
            28 => CauseOfDeath::Unknown,
            _ => return Err(Error::CauseOfDeathFromByte(value)),
        };

        Ok(cause)
    }
}

impl FromStr for CauseOfDeath {
    type Err = crate::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let cause = match input {
            "MOD_SHOTGUN" => CauseOfDeath::Shotgun,
            "MOD_GAUNTLET" => CauseOfDeath::Gauntlet,
            "MOD_MACHINEGUN" => CauseOfDeath::Machinegun,
            "MOD_GRENADE" => CauseOfDeath::Grenade,
            "MOD_GRENADE_SPLASH" => CauseOfDeath::GrenadeSplash,
            "MOD_ROCKET" => CauseOfDeath::Rocket,
            "MOD_ROCKET_SPLASH" => CauseOfDeath::RocketSplash,
            "MOD_PLASMA" => CauseOfDeath::Plasma,
            "MOD_PLASMA_SPLASH" => CauseOfDeath::PlasmaSplash,
            "MOD_RAILGUN" => CauseOfDeath::Railgun,
            "MOD_LIGHTNING" => CauseOfDeath::Lightning,
            "MOD_BFG" => CauseOfDeath::Bfg,
            "MOD_BFG_SPLASH" => CauseOfDeath::BfgSplash,
            "MOD_WATER" => CauseOfDeath::Water,
            "MOD_SLIME" => CauseOfDeath::Slime,
            "MOD_LAVA" => CauseOfDeath::Lava,
            "MOD_CRUSH" => CauseOfDeath::Crush,
            "MOD_TELEFRAG" => CauseOfDeath::Telefrag,
            "MOD_FALLING" => CauseOfDeath::Falling,
            "MOD_SUICIDE" => CauseOfDeath::Suicide,
            "MOD_TARGET_LASER" => CauseOfDeath::TargetLaser,
            "MOD_TRIGGER_HURT" => CauseOfDeath::TriggerHurt,
            "MOD_NAIL" => CauseOfDeath::Nail,
            "MOD_CHAINGUN" => CauseOfDeath::Chaingun,
            "MOD_PROXIMITY_MINE" => CauseOfDeath::ProximityMine,
            "MOD_KAMIKAZE" => CauseOfDeath::Kamikaze,
            "MOD_JUICED" => CauseOfDeath::Juiced,
            "MOD_GRAPPLE" => CauseOfDeath::Grapple,
            "MOD_UNKNOWN" => CauseOfDeath::Unknown,
            _ => {
                return Err(Error::UnknownCauseOfDeath(
                    input.into(),
                ))
            }
        };

        Ok(cause)
    }
}

impl AsRef<str> for CauseOfDeath {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for CauseOfDeath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use strum::{EnumCount, IntoEnumIterator};

    use super::{CauseOfDeath, CAUSES_OF_DEATH};

    /// Converts a mean of death into the expected
    /// Quake format
    ///
    /// E.g.:
    ///     `CauseOfDeath::Rocket` becomes "MOD_ROCKET"
    ///     `CauseOfDeath::BfgSplash` becomes "MOD_BFG_SPLASH"
    fn to_quake_format(cause: CauseOfDeath) -> String {
        // Emulate `serde_plain::to_string` using Debug
        let debug_msg = format!("{:?}", cause);

        let as_shouty = heck::AsShoutySnakeCase(debug_msg);

        format!("MOD_{as_shouty}")
    }

    #[test]
    fn has_correct_total_of_causes_of_death() {
        assert_eq!(CAUSES_OF_DEATH, CauseOfDeath::COUNT);
    }

    #[test]
    fn parses_cause_of_death_from_str() {
        let input =
            CauseOfDeath::iter().map(CauseOfDeath::as_str);
        let expected = CauseOfDeath::iter();

        // TODO: is this a good test?
        for (string_representation, expected_cause_of_death) in
            input.zip(expected)
        {
            assert_eq!(
                string_representation
                    .parse::<CauseOfDeath>()
                    .unwrap(),
                expected_cause_of_death
            )
        }
    }

    #[test]
    /// Ensures that `CauseOfDeath::as_str` returns
    /// a String in the expected Quake format
    fn to_str_adds_mod_prefix() {
        for cause_of_death in CauseOfDeath::iter() {
            assert_eq!(
                cause_of_death.as_str(),
                to_quake_format(cause_of_death)
            );
        }
    }

    #[test]
    fn check_into_u8_impl() {
        let expected_numbers = 0..CauseOfDeath::COUNT;

        for (cause_of_death, expected_number) in
            CauseOfDeath::iter().zip(expected_numbers)
        {
            let byte: u8 = cause_of_death.into();
            assert_eq!(byte as usize, expected_number);
        }
    }

    #[test]
    fn check_from_u8_impl() {
        for (idx, expected_cause) in
            (0..CauseOfDeath::COUNT).zip(CauseOfDeath::iter())
        {
            let cause_of_death =
                CauseOfDeath::try_from(idx as u8).unwrap();

            assert_eq!(cause_of_death, expected_cause);
        }
    }
}
