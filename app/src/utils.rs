pub fn emojify(original: impl AsRef<str>) -> String {
    let original = original.as_ref();
    let mut result = String::new();
    let mut emoji = String::new();

    for c in original.chars() {
        if c == ':' {
            emoji.push(c);

            if emoji.len() > 1 {
                result.push_str(
                    match emojis::get_by_shortcode(&emoji.as_str().replace(":", "")) {
                        Some(emoji) => emoji.as_str(),
                        None => emoji.as_str(),
                    },
                );

                emoji.clear();
            }
        } else if emoji.len() > 0 {
            emoji.push(c);
        } else {
            result.push(c);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_emojis() {
        let result = emojify("Do you like :sushi: sushi?");
        assert_eq!(result, "Do you like 🍣 sushi?");
    }

    #[test]
    fn it_leaves_unsupported_emojis() {
        let result = emojify("Do you like :not_a_supported_emoji: sushi?");
        assert_eq!(result, "Do you like :not_a_supported_emoji: sushi?");
    }

    #[test]
    fn it_handles_adjacent_punctuation() {
        let result = emojify("Hello :globe_showing_Americas:!");
        assert_eq!(result, "Hello 🌎!");
    }
}
