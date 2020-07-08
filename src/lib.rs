#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod parser;
pub use parser::parse_key_string;

mod platform;
pub use platform::{KeyModifier, KeySpecial};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Key {
    Alphanumeric(char),
    Modifier(KeyModifier),
    Special(KeySpecial),
}
