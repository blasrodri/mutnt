//! Parser ...

use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_till, take_until},
    character::{complete::multispace1, is_space},
    error::Error as NomError,
    Finish, IResult,
};

use crate::{
    command::{Command, CommandBuilder},
    errors::QueryError,
};

#[derive(Debug, PartialEq)]
enum Token {
    // CommandAction
    Get,
    Insert,
    Delete,

    // Query
    QueryFirst,
    QueryLast,
    QueryKey(String),
    QueryIndex(usize),

    // Datastructure
    Datastructure(String),
}

// gets the `Token` and removes subsequent whitespaces
fn get_command_action(input: &[u8]) -> IResult<&[u8], Token> {
    alt((is_query, is_insert, is_delete))(input).and_then(|(i, token)| {
        let (input, _) = multispace1(i)?;
        Ok((input, token))
    })
}

fn get_datastructure_name(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("FROM")(input).and_then(|(i, _)| multispace1(i))?;
    let (input, data_structure_name) = take_till(is_space)(input)?;
    let (input, _) = multispace1(input)?;
    Ok((
        input,
        Token::Datastructure(String::from_utf8(data_structure_name.to_ascii_lowercase()).unwrap()),
    ))
}

fn get_query_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((get_query_first, get_query_last))(input)
}

fn get_query_first(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("FIRST")(input)?;
    Ok((input, Token::QueryFirst))
}

fn get_query_last(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("LAST")(input)?;
    Ok((input, Token::QueryLast))
}

fn get_query_key(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("WHERE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("KEY")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("=")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, key) = take_until(";")(input)?;
    Ok((
        input,
        Token::QueryKey(String::from_utf8(key.to_ascii_lowercase().to_vec()).unwrap()),
    ))
}

fn get_query_index(input: &[u8]) -> IResult<&[u8], Token> {
    let (input, _) = tag_no_case("WHERE")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("INDEX")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag_no_case("=")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, idx_u8) = take_until(";")(input)?;
    let idx_str = String::from_utf8(idx_u8.to_ascii_lowercase().to_vec()).unwrap();

    Ok((input, Token::QueryIndex(idx_str.parse().unwrap())))
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
pub fn parse(input: &[u8]) -> Result<Command, QueryError> {
    let (input, _command_action_token) = get_command_action(input)
        .finish()
        .map_err(|e| QueryError::NomError(e.code))?;
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
            assert_eq!((b"FROM my_datastructure".as_ref(), token), result)
        }
    }

    #[test]
    fn parse_datastructure_name() {
        // NOTE: newlines will break
        for input in [
            "FROM my_datastructure ",
            "FROM   my_datastructure ",
            "FROM   \nmy_datastructure ",
        ] {
            let result = get_datastructure_name(input.as_ref()).unwrap();
            assert_eq!(
                (
                    b"".as_ref(),
                    Token::Datastructure("my_datastructure".to_string())
                ),
                result
            )
        }
    }
}
