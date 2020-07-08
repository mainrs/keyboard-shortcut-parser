//! This crate allows to parse keyboard shortcut strings and maps the results to the right keycodes.
//! Keycode mapping is currently supported for X11 and Windows.

#![deny(
    missing_docs,
    missing_copy_implementations,
    missing_debug_implementations,
    unsafe_code,
)]
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod parser;
pub use parser::parse_key_string;

mod platform;
pub use platform::{KeyModifier, KeySpecial};

/// A keyboard key.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Key {
    /// An alphanumeric key ([a-zA-Z0-9]).
    Alphanumeric(char),
    /// A modifier key (Alt, Control, Shift or Super).
    Modifier(KeyModifier),
    /// A special key (anything that is not one of the above).
    Special(KeySpecial),
}
