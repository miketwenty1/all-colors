pub mod colors;

pub fn get_color_hex(color: &str) -> Option<&str> {
    let color_key = sanitize_color_string(color);
    let binding = colors::get_colors();
    let a = binding.get(color_key.as_str());
    a.copied()
}
fn sanitize_color_string(color: &str) -> String {
    let sanitized = color
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<String>()
        .to_lowercase();

    sanitized
}
#[cfg(test)]
mod tests {
    use crate::get_color_hex;

    #[test]
    fn test_aqua() {
        let result = get_color_hex("aqua");
        assert_eq!(result.unwrap(), "00ffff");
    }
    #[test]
    fn test_removing_bad_chars() {
        let result = get_color_hex("aqua2[]");
        assert_eq!(result.unwrap(), "00ffff");
    }
    #[test]
    fn test_complicated() {
        let result = get_color_hex("Purple Mountain's Majesty");
        assert_eq!(result.unwrap(), "9d81ba");
    }
    #[test]
    fn test_none() {
        let result = get_color_hex("snow white");
        assert_eq!(result, None);
    }
}
