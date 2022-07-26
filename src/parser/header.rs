use nom::{
    branch::alt, bytes::complete::tag, combinator::value,
    IResult,
};

use super::combinator::{
    parse_spacer_line, parse_timestamp, parse_ws,
};

/// A possible "header" for a line within the Quake 3 Arena log
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Header {
    InitGame,
    Kill,
    ClientBegin,
    ClientUserinfoChanged,
    ClientConnect,
    ShutdownGame,
    Score,
    Item,
    Exit,
    /// Not an actual action but represents the "spacer" line
    /// that comes after ShutdownGame and before InitGame
    ///
    /// ```no-rust
    /// 981:39 ShutdownGame:
    /// 981:39 ------------------------------------------------------------
    ///   0:00 ------------------------------------------------------------
    /// ```
    Spacer,
}

pub fn parse_header(input: &str) -> IResult<&str, Header> {
    let (rest, _) = parse_timestamp(input)?;
    let (rest, _ws) = parse_ws(rest)?;

    alt((
        value(Header::Kill, tag("Kill")),
        value(Header::InitGame, tag("InitGame")),
        value(Header::ClientBegin, tag("ClientBegin")),
        value(
            Header::ClientUserinfoChanged,
            tag("ClientUserinfoChanged"),
        ),
        value(Header::ClientConnect, tag("ClientConnect")),
        value(Header::ShutdownGame, tag("ShutdownGame")),
        value(Header::Score, tag("score")),
        value(Header::Exit, tag("Exit")),
        value(Header::Item, tag("Item")),
        value(Header::Spacer, parse_spacer_line),
    ))(rest)
}

#[cfg(test)]
mod tests {
    use super::{parse_header, Header};

    #[test]
    fn parses_headers() {
        assert_eq!(
            parse_header("  1:53 ------------------------------------------------------------"),
            Ok(("", Header::Spacer))
        );

        assert_eq!(
            parse_header("  1:47 ClientBegin: 3"),
            Ok((": 3", Header::ClientBegin))
        );

        assert_eq!(
            parse_header("  0:00 InitGame: \\capturelimit\\8"),
            Ok((": \\capturelimit\\8", Header::InitGame))
        );

        assert_eq!(
            parse_header("  27:14 Exit: Timelimit hit."),
            Ok((": Timelimit hit.", Header::Exit))
        );

        assert_eq!(
            parse_header("  2:33 Item: 4 ammo_shells"),
            Ok((": 4 ammo_shells", Header::Item))
        );

        assert_eq!(
            parse_header("54:21 ShutdownGame:"),
            Ok((":", Header::ShutdownGame))
        );

        assert_eq!(
            parse_header(
                "11:57 score: 20  ping: 4  client: 4 Zeh"
            ),
            Ok((": 20  ping: 4  client: 4 Zeh", Header::Score))
        );

        assert_eq!(
            parse_header("  2:43 Kill: 3 4 10: Isgalamido killed Zeh by MOD_RAILGUN"),
            Ok((
                ": 3 4 10: Isgalamido killed Zeh by MOD_RAILGUN",
                Header::Kill
            ))
        );
    }
}
