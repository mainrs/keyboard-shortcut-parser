use crate::platform::{KeyModifier, KeySpecial};
use crate::Key;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::alphanumeric1;
use nom::combinator::map;
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
    let key_parser = map(
        alphanumeric1,
        |s: &str| {
            if s.len() == 1 {
                // Simple alphanumeric char.
                return Key::Alphanumeric(s.chars().next().unwrap());
            }

            let converted_to_special_key = KeySpecial::from_str(s);
            if converted_to_special_key.is_ok() {
                return Key::Special(converted_to_special_key.unwrap());
            } else {
                panic!("Not a valid special key or alphanumeric key: {}", s);
            }
        }
    );
    alt((modifier_parser, key_parser))(i)
}

/// Parses a string and returns the list of recognized keys, in order.
///
/// # Panics
///
/// Panics if the keystring can't be parsed correctly. This happens if the segments are not valid modifiers, special keys or are alphanumeric and longer than one char.
///
/// # Examples
///
/// ```
/// use keyboard_shortcut_parser::{parse_key_string, Key, KeyModifier, KeySpecial};
///
/// let i = "ctrl+alt+delete";
/// let v = parse_key_string(&i).unwrap().1;
/// assert_eq!(v, vec![Key::Modifier(KeyModifier::CONTROL), Key::Modifier(KeyModifier::ALT), Key::Special(KeySpecial::DELETE)])
/// ```
pub fn parse_key_string(i: &str) -> IResult<&str, Vec<Key>> {
    separated_list1(tag("+"), parse_key)(i)
}
