use std::convert::TryFrom;
use std::str::FromStr;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::{WinKeyModifier as KeyModifier, WinKeySpecial as KeySpecial};

#[cfg(all(linux, feature = "linux-x11"))]
mod x11;
#[cfg(all(linux, feature = "linux-x11"))]
pub use x11::{X11KeyModifier as KeyModifier, X11KeySpecial as KeySpecial};

impl From<KeyModifier> for &str {
    fn from(km: KeyModifier) -> Self {
        match km {
            KeyModifier::ALT => "alt",
            KeyModifier::CONTROL => "control",
            KeyModifier::SHIFT => "shift",
            KeyModifier::SUPER => "super",
            _ => unreachable!(),
        }
    }
}

impl<'a> TryFrom<&'a str> for KeyModifier {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for KeyModifier {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modifier = match s.to_lowercase().as_str() {
            "alt" => KeyModifier::ALT,
            "control" => KeyModifier::CONTROL,
            "shift" => KeyModifier::SHIFT,
            "super" => KeyModifier::SUPER,
            _ => panic!("Not a valid key modifier: {}", s),
        };

        Ok(modifier)
    }
}

impl From<KeySpecial> for &str {
    fn from(ks: KeySpecial) -> Self {
        match ks {
            KeySpecial::BACKSPACE => "backspace",
            KeySpecial::TAB => "tab",
            KeySpecial::CLEAR => "clear",
            KeySpecial::ENTER => "enter",
            KeySpecial::PAUSE => "pause",
            KeySpecial::CAPSLOCK => "capslock",
            KeySpecial::ESCAPE => "escape",
            KeySpecial::SPACEBAR => "spacebar",
            KeySpecial::PAGE_UP => "pageup",
            KeySpecial::PAGE_DOWN => "pagedown",
            KeySpecial::END => "end",
            KeySpecial::HOME => "home",
            KeySpecial::LEFT_ARROW => "left",
            KeySpecial::UP_ARROW => "up",
            KeySpecial::RIGHT_ARROW => "right",
            KeySpecial::DOWN_ARROW => "down",
            KeySpecial::SELECT => "select",
            KeySpecial::PRINT => "print",
            KeySpecial::PRINT_SCREEN => "printscreen",
            KeySpecial::INSERT => "insert",
            KeySpecial::DELETE => "delete",
            KeySpecial::F1 => "f1",
            KeySpecial::F2 => "f2",
            KeySpecial::F3 => "f3",
            KeySpecial::F4 => "f4",
            KeySpecial::F5 => "f5",
            KeySpecial::F6 => "f6",
            KeySpecial::F7 => "f7",
            KeySpecial::F8 => "f8",
            KeySpecial::F9 => "f9",
            KeySpecial::F10 => "f10",
            KeySpecial::F11 => "f11",
            KeySpecial::F12 => "f12",
            KeySpecial::NUMLOCK => "numlock",
            KeySpecial::SCROLLLOCK => "scrolllock",
            _ => unreachable!(),
        }
    }
}

impl<'a> TryFrom<&'a str> for KeySpecial {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for KeySpecial {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s.to_lowercase().as_str() {
            "backspace" => KeySpecial::BACKSPACE,
            "tab" => KeySpecial::TAB,
            "clear" => KeySpecial::CLEAR,
            "enter" => KeySpecial::ENTER,
            "pause" => KeySpecial::PAUSE,
            "capslock" => KeySpecial::CAPSLOCK,
            "escape" => KeySpecial::ESCAPE,
            "spacebar" => KeySpecial::SPACEBAR,
            "pageup" => KeySpecial::PAGE_UP,
            "pagedown" => KeySpecial::PAGE_DOWN,
            "end" => KeySpecial::END,
            "home" => KeySpecial::HOME,
            "left" => KeySpecial::LEFT_ARROW,
            "up" => KeySpecial::UP_ARROW,
            "right" => KeySpecial::RIGHT_ARROW,
            "down" => KeySpecial::DOWN_ARROW,
            "select" => KeySpecial::SELECT,
            "print" => KeySpecial::PRINT,
            "printscreen" => KeySpecial::PRINT_SCREEN,
            "insert" => KeySpecial::INSERT,
            "delete" => KeySpecial::DELETE,
            "f1" => KeySpecial::F1,
            "f2" => KeySpecial::F2,
            "f3" => KeySpecial::F3,
            "f4" => KeySpecial::F4,
            "f5" => KeySpecial::F5,
            "f6" => KeySpecial::F6,
            "f7" => KeySpecial::F7,
            "f8" => KeySpecial::F8,
            "f9" => KeySpecial::F9,
            "f10" => KeySpecial::F10,
            "f11" => KeySpecial::F11,
            "f12" => KeySpecial::F12,
            "numlock" => KeySpecial::NUMLOCK,
            "scrolllock" => KeySpecial::SCROLLLOCK,
            _ => panic!("Not a valid special key: {}", s),
        };

        Ok(key)
    }
}
