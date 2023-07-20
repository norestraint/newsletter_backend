use unicode_segmentation::UnicodeSegmentation;
pub struct SubscriberName(String);

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contain_forbidden_characters = s.chars().any(|ch| forbidden_characters.contains(&ch));

        if is_empty_or_whitespace || is_too_long || contain_forbidden_characters {
            panic!("{}", format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::domain::SubscriberName;

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "a".repeat(256);
        assert_eq!(Ok(SubscriberName), SubscriberName::parse(name))
    }
}
