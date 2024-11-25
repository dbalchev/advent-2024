use regex::Regex;

pub fn make_regex(regex_text: &'static str) -> Regex {
    Regex::new(regex_text).unwrap()
}
