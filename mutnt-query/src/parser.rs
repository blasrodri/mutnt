//! Parser ...

use nom::{branch::alt, bytes::complete::tag_no_case, IResult};

use crate::{
    command::{Command, CommandBuilder},
    errors::Error,
};
pub struct Parser;

#[derive(Debug, PartialEq)]
enum Token {
    // CommandAction
    Get,
    Insert,
    Delete,
}

fn get_command_action(input: &[u8]) -> IResult<&[u8], Token> {
    alt((is_query, is_insert, is_delete))(input)
}

fn is_query(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("GET")(input)?;
    Ok((input, Token::Get))
}

fn is_insert(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("INSERT")(input)?;
    Ok((input, Token::Insert))
}

fn is_delete(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("DELETE")(input)?;
    Ok((input, Token::Delete))
}

/// Entrypoint to parse a command
pub fn parse(_input: &[u8]) -> Result<Command, Error> {
    let builder = CommandBuilder::new();
    builder.build_perhaps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_command_action() {
        for (action, token) in [
            ("GET", Token::Get),
            ("INSERT", Token::Insert),
            ("DELETE", Token::Delete),
        ] {
            let input = format!("{} FROM my_datastructure", action);
            let result = get_command_action(input.as_ref()).unwrap();
            assert_eq!((b" FROM my_datastructure".as_ref(), token), result)
        }
    }
}
