# all-colors
rust crate that returns a hex for a color name

For a full list of supported colors check colors.rs.
Inputs are lowercased and sanitized before comparing.

```rs
use all_colors::get_color_hex;

let color_hex = get_color_hex("aqua").expect("a color hex str for aqua");

println!("aqua is #{}", color_hex);
```