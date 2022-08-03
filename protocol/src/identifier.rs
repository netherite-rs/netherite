use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"[a-z0-9._-]+").unwrap();
    static ref VALUE_REGEX: Regex = Regex::new("[a-z0-9.-_/]+").unwrap();
}

static MINECRAFT: &str = "minecraft";

pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn minecraft(value: String) -> Self {
        Self::new(String::from(MINECRAFT), value)
    }

    pub fn new(namespace: String, value: String) -> Self {
        if !NAMESPACE_REGEX.is_match(&namespace) {
            panic!("Namespace '{}' can only contain lowercase alphabet, underscores, dots, dashes and numbers.", namespace)
        }
        if !VALUE_REGEX.is_match(&value) {
            panic!("Value '{}' can only contain lowercase alphabet, underscores, dots, dashes, slashes and numbers.", namespace)
        }
        let string = format!("{}:{}", namespace, value);
        if string.len() >= 256 {
            panic!("Identifiers must be less than 256 characters")
        }
        Self { namespace, value }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
