pub mod colors;
use sha2::{Digest, Sha256};
use std::fmt::Write;

pub fn get_color_hex(color: &str) -> String {
    let color_key = sanitize_color_string(color);
    let color_lookedup = colors::COLORS.get(color_key.as_str()).copied();

    // if None see if we can do something clever to come up with a color
    match color_lookedup {
        Some(s) => s.to_string(),
        None => come_up_with_color(&color_key),
    }
}

fn sanitize_color_string(color: &str) -> String {
    let sanitized = color
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
        .to_lowercase();

    sanitized
}

fn come_up_with_color(color: &str) -> String {
    let mut result = String::with_capacity(6);
    // check to see if the str is 3 or 6 length AND all chars are hex chars
    if (color.len() == 6 || color.len() == 3) && color.chars().all(|c| c.is_ascii_hexdigit()) {
        if color.len() == 3 {
            for ch in color.chars() {
                result.push(ch);
                result.push(ch);
            }
        } else {
            result = color.to_owned();
        }
        // Check to see if a bunch of numbers were given 0-255,0-255,0-255,
    } else if color.chars().all(|c| c.is_ascii_digit()) && color.len() == 9 {
        let part1 = &color[0..3];
        let part2 = &color[3..6];
        let part3 = &color[6..9];

        let part1_num = part1.parse::<u8>().ok().unwrap_or(255);
        let part2_num = part2.parse::<u8>().ok().unwrap_or(255);
        let part3_num = part3.parse::<u8>().ok().unwrap_or(255);

        let hex_part1 = format!("{:02x}", part1_num);
        let hex_part2 = format!("{:02x}", part2_num);
        let hex_part3 = format!("{:02x}", part3_num);

        result = format!("{}{}{}", hex_part1, hex_part2, hex_part3);
    } else {
        // hash whatever we have then just return 6 hex chars for a color
        let mut hasher = Sha256::new();
        hasher.update(color);
        let hresult = hasher.finalize();
        // let hex_string: String = hresult
        //     .iter()
        //     .take(3) // Take the first 3 bytes (6 hex characters)
        //     .map(|byte| format!("{:02x}", byte))
        //     .collect();
        let hex_string: String = hresult
            .iter()
            .take(3) // Take the first 3 bytes (6 hex characters)
            .fold(String::new(), |mut acc, byte| {
                // Use write! to append formatted text directly to the string
                write!(acc, "{:02x}", byte).expect("Can't write to string");
                acc // Return the accumulator for the next fold iteration
            });
        result = hex_string;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::get_color_hex;

    #[test]
    fn test_aqua() {
        let result = get_color_hex("aqua");
        assert_eq!(result, "00ffff");
    }
    #[test]
    fn test_removing_bad_chars() {
        let result = get_color_hex("aqua[]");
        assert_eq!(result, "00ffff");
    }
    #[test]
    fn test_complicated() {
        let result = get_color_hex("Purple Mountain's Majesty");
        assert_eq!(result, "9d81ba");
    }
    #[test]
    fn test_hasher() {
        let result = get_color_hex("snow white");
        // will test "snowwhite"
        assert_eq!(result, "b87df3");
    }
    #[test]
    fn test_plainhex6() {
        let result = get_color_hex("dabeef");
        assert_eq!(result, "dabeef");
    }
    #[test]
    fn test_plainhex3() {
        let result = get_color_hex("1fa");
        assert_eq!(result, "11ffaa");
    }
    #[test]
    fn test_high_digits_truncating() {
        let result = get_color_hex("999999999");
        assert_eq!(result, "ffffff");
    }
    #[test]
    fn test_digits() {
        let result = get_color_hex("010255001");
        assert_eq!(result, "0aff01");
    }
    #[test]
    fn test_longrandomstring() {
        let result = get_color_hex("give me something interesting");
        assert_eq!(result, "a4f662");
    }
    #[test]
    fn test_empty() {
        let result = get_color_hex("");
        assert_eq!(result, "e3b0c4");
    }
}
