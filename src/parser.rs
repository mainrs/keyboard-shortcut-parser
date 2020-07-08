use crate::platform::KeyModifier;
use crate::Key;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::alphanumeric1;
use nom::combinator::{map, verify};
use nom::multi::separated_list1;
use nom::IResult;
use std::str::FromStr;

fn parse_alt_key(i: &str) -> IResult<&str, KeyModifier> {
    let parser = tag_no_case(KeyModifier::ALT.into());
    // Safe unwrap as nom takes care of actually having the right tag here.
    map(parser, |t: &str| KeyModifier::from_str(t).unwrap())(i)
}

fn parse_ctrl_key(i: &str) -> IResult<&str, KeyModifier> {
    let parser = alt((
        tag_no_case("ctrl"),
        tag_no_case(KeyModifier::CONTROL.into()),
    ));
    // Safe unwrap as nom takes care of actually having the right tag here.
    map(parser, |t: &str| KeyModifier::from_str(t).unwrap())(i)
}

fn parse_super_key(i: &str) -> IResult<&str, KeyModifier> {
    let parser = tag_no_case(KeyModifier::SUPER.into());
    // Safe unwrap as nom takes care of actually having the right tag here.
    map(parser, |t: &str| KeyModifier::from_str(t).unwrap())(i)
}

fn parse_modifier_key(i: &str) -> IResult<&str, KeyModifier> {
    alt((parse_alt_key, parse_ctrl_key, parse_super_key))(i)
}

fn parse_key(i: &str) -> IResult<&str, Key> {
    let modifier_parser = map(parse_modifier_key, |km| Key::Modifier(km));
    let char_parser = verify(alphanumeric1, |s: &str| s.len() == 1);
    let alphanumeric_parser = map(char_parser, |an: &str| {
        // Safe unwrap as the size is checked beforehand.
        Key::Alphanumeric(an.chars().next().unwrap())
    });
    alt((modifier_parser, alphanumeric_parser))(i)
}

pub fn parse_key_string(i: &str) -> IResult<&str, Vec<Key>> {
    separated_list1(tag("+"), parse_key)(i)
}
