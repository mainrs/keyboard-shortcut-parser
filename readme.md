# keyboard-shortcut-parser

> A small Rust library used to parse shortcut strings like `ctrl+alt+p`.

## Example 

```rust
use keyboard_shortcut_parser::{parse_key_string, Key, KeyModifier, KeySpecial};

let i = "ctrl+alt+delete";
let v: Vec<Key> = parse_key_string(&i).unwrap().1;
```
