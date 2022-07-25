use std::fmt::Display;

#[cfg(test)]
use strum::IntoEnumIterator;

// means of death
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(test, derive(strum::EnumIter))]
enum CauseOfDeath {
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
    pub fn as_str(&self) -> &'static str {
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

    use super::CauseOfDeath;

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
    /// Ensures that `CauseOfDeath::as_str` returns
    /// a String in the expected Quake format
    fn to_str_adds_mod_prefix() {
        use strum::IntoEnumIterator;

        for mean in CauseOfDeath::iter() {
            assert_eq!(mean.as_str(), to_quake_format(mean));
        }
    }
}
