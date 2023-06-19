# all-colors
rust crate for converting a &str into a hex string


`get_color_hex` takes in a &str and will return a 6 char hex string.

# Inputs this function will accept:
- 6 digit hex string will return the original hex string. 
- 3 digit hex will return 6 char hex string.
- A standard color name from a list of roughly ~750 in colors.rs.
- 9 digit decimal string, if over (0-255),(0-255),(0-255) will use 255.
- Otherwise will sha256 hash the string and grab the first 3 bytes.

```rust
fn main() {
    use all_colors::get_color_hex;

    //EXAMPLES:
    let color_hex = get_color_hex("aqua").expect("well known color name");
    let color_hex2 = get_color_hex("give me something interesting").expect("some random color a4f662");
    let color_hex3 = get_color_hex("f20").expect("f00 will return ff2200");
    let color_hex4 = get_color_hex("010255001").expect("9 digits will become 0aff01");



    println!("aqua is #{}", color_hex);
}





```