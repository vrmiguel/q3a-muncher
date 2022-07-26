use std::str::FromStr;

use nom::{
    bytes::complete::{
        tag, take_till1, take_until1, take_while, take_while_m_n,
    },
    character::complete::char,
    sequence::{
        delimited, preceded, separated_pair, terminated, tuple,
    },
    IResult,
};

use crate::CauseOfDeath;

#[derive(Debug, PartialEq, Eq)]
/// The information contained in a kill message, such as
/// "<world> killed Isgalamido by MOD_TRIGGER_HURT"
///  ∧∧∧∧∧∧         ∧∧∧∧∧∧∧∧∧∧    ∧∧∧∧∧∧∧∧∧∧∧∧∧∧∧∧
///  attacker         victim       cause of death
pub struct KillMessage<'a> {
    pub attacker: &'a str,
    pub victim: &'a str,
    pub cause_of_death: CauseOfDeath,
}

// TODO: breaks if `attacker`'s name contains " killed " and if
//       `victim`'s name contains " by "
pub fn parse_kill_message(
    input: &str,
) -> IResult<&str, KillMessage> {
    const KILLED_TAG: &str = " killed ";
    const BY_TAG: &str = " by ";

    let (rest, attacker) = preceded(
        parse_ws,
        terminated(take_until1(KILLED_TAG), tag(KILLED_TAG)),
    )(input)?;

    let (rest, victim) = preceded(
        parse_ws,
        terminated(take_until1(BY_TAG), tag(BY_TAG)),
    )(rest)?;

    let (rest, cause_of_death) =
        take_till1(is_ascii_whitespace)(rest)?;

    let cause_of_death = CauseOfDeath::from_str(cause_of_death)
        .or_else(|_| {
            use nom::error::{ContextError, Error, ErrorKind};

            // help me god
            Err(nom::Err::Error(ContextError::add_context(
                cause_of_death,
                "Not a valid CauseOfDeath",
                Error::new(cause_of_death, ErrorKind::Tag),
            )))
        })?;

    let kill_message = KillMessage {
        attacker,
        victim,
        cause_of_death,
    };

    Ok((rest, kill_message))
}

pub fn parse_kill(input: &str) -> IResult<&str, KillMessage> {
    let (rest, _) = parse_kill_metadata(input)?;
    parse_kill_message(rest)
}

/// Parses timestamps in the form of `MM:ss`.
///
/// `MM` and `ss` are returned as string slices.
pub fn parse_timestamp(
    input: &str,
) -> IResult<&str, (&str, &str)> {
    separated_pair(
        preceded(parse_ws, parse_decimals),
        char(':'),
        parse_decimals,
    )(input)
}

/// Parses a spacer line: that is, a line comprised of
/// a sequence of 60 hyphens
#[inline(always)]
pub fn parse_spacer_line(input: &str) -> IResult<&str, &str> {
    let is_hyphen = |ch: char| ch == '-';

    take_while_m_n(60, 60, is_hyphen)(input)
}

/// Parses the "metadata" for a log line describing a kill, that
/// is, the information contained after the kill header, as shown
/// below:
///
///            ∨∨∨∨∨∨∨∨∨∨∨∨
/// "20:54 Kill: 1022 2 22: <world> killed Isgalamido by"
///
/// Returns the three numbers within the metadata as string
/// slices
pub fn parse_kill_metadata(
    input: &str,
) -> IResult<&str, (&str, &str, &str)> {
    let parse_triplet = tuple((
        preceded(parse_ws, parse_decimals),
        char(' '),
        parse_decimals,
        char(' '),
        parse_decimals,
    ));

    let (rest, (first_digit, _, second_digit, _, third_digit)) =
        delimited(parse_colon, parse_triplet, parse_colon)(
            input,
        )?;

    Ok((rest, (first_digit, second_digit, third_digit)))
}

#[inline(always)]
fn parse_colon(input: &str) -> IResult<&str, char> {
    preceded(parse_ws, char(':'))(input)
}

#[inline(always)]
fn parse_decimals(input: &str) -> IResult<&str, &str> {
    let is_decimal = |ch: char| ch.is_digit(10);

    take_while(is_decimal)(input)
}

/// Parses leading whitespace
#[inline(always)]
pub fn parse_ws(input: &str) -> IResult<&str, &str> {
    take_while(is_ascii_whitespace)(input)
}

#[inline(always)]
const fn is_ascii_whitespace(ch: char) -> bool {
    matches!(ch, '\t' | '\n' | '\x0C' | '\r' | ' ')
}

#[cfg(test)]
mod tests {

    use super::{
        parse_kill_message, parse_kill_metadata,
        parse_timestamp, KillMessage,
    };
    use crate::CauseOfDeath;

    #[test]
    fn parses_kill_message() {
        assert_eq!(
            parse_kill_message(
                "<world> killed Isgalamido by MOD_TRIGGER_HURT"
            ),
            Ok((
                "",
                KillMessage {
                    attacker: "<world>",
                    victim: "Isgalamido",
                    cause_of_death: CauseOfDeath::TriggerHurt,
                }
            ))
        );

        assert_eq!(
            parse_kill_message(
                "<world> killed Dono da Bola by MOD_FALLING"
            ),
            Ok((
                "",
                KillMessage {
                    attacker: "<world>",
                    victim: "Dono da Bola",
                    cause_of_death: CauseOfDeath::Falling,
                }
            ))
        );
    }

    #[test]
    fn parses_timestamps() {
        assert_eq!(
            parse_timestamp(" 5:40"),
            Ok(("", ("5", "40")))
        );

        assert_eq!(
            parse_timestamp("20:00 "),
            Ok((" ", ("20", "00")))
        );
    }

    #[test]
    fn parses_kill_metadata() {
        assert_eq!(
            parse_kill_metadata(": 1022 2 22:"),
            Ok(("", ("1022", "2", "22")))
        )
    }
}
