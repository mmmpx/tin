use std::collections::HashMap;

use nom::{
    IResult,
    bytes::complete::{is_not, take_till},
    character::complete::{ char, space1, digit1, alpha1, crlf },
    sequence::{ separated_pair, preceded, delimited },
    multi::{ separated_list1, separated_list0 },
    branch::alt,
    combinator::opt,
};

// Struct used for holding Twitch IRC message data, partly based on the example
// available at https://dev.twitch.tv/docs/irc/example-parser.

// https://modern.ircdocs.horse/#client-to-server-protocol-structure
// message         ::= ['@' <tags> SPACE] [':' <source> SPACE] <command> <parameters> <crlf>
// SPACE           ::=  %x20 *( %x20 )   ; space character(s)
// crlf            ::=  %x0D %x0A        ; "carriage return" "linefeed"

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub tags: Option<HashMap<String, String>>,
    pub source: Option<String>,
    pub command: String,
    pub parameters: Vec<String>
}

fn tags(s: &str) -> IResult<&str, Vec<(&str, &str)>> {

    fn tag_(s: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(is_not("="), char('='), take_till(|c| c == ' ' || c == ';'))(s)
    }

    fn tags_(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
        separated_list1(char(';'), tag_)(s)
    }

    delimited(char('@'), tags_, space1)(s)
}

fn source(s: &str) -> IResult<&str, &str> {
    delimited(char(':'), is_not(" "), space1)(s)
}

fn command(s: &str) -> IResult<&str, &str> {
    alt((alpha1, digit1))(s)
}

fn parameters(s: &str) -> IResult<&str, Vec<&str>> {

    fn trailing(s: &str) -> IResult<&str, &str> {
        preceded(char(':'), is_not("\r\n"))(s)
    }

    let (s, mut params) = separated_list0(space1, is_not(" \r\n:"))(s)?;

    let (s, tr) = if params.len() > 0 {
        opt(preceded(space1, trailing))(s)?
    } else {
        opt(trailing)(s)?
    };

    if let Some(tr_) = tr { params.push(tr_); }
    Ok((s, params))
}

pub fn message(s: &str) -> IResult<&str, Message> {
    let (s, tags) = opt(tags)(s)?;
    let (s, source) = opt(source)(s)?;
    let (s, command) = command(s)?;
    let (s, parameters) = delimited(space1, parameters, crlf)(s)?;

    let m = Message {
        tags: tags.map(|ts| ts.into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()),
        source: source.map(str::to_string),
        command: command.to_string(),
        parameters: parameters.into_iter().map(str::to_string).collect()
    };

    Ok((s, m))
}
