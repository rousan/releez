use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::env;

lazy_static! {
    static ref VARIABLE_RE: Regex = Regex::new(r"(?:\$([a-zA-Z\d_]+))|(?:\$\{\s*([a-zA-Z\d_]+)\s*\})").unwrap();
}

pub fn substitute_variable_in_text(text: &str, vars_data: &HashMap<String, String>) -> String {
    VARIABLE_RE
        .replace_all(text, |caps: &Captures| {
            let var_name = caps
                .get(1)
                .or_else(|| caps.get(2))
                .expect("Failed to extract variable name from text")
                .as_str();

            vars_data
                .get(var_name)
                .map(|v| v.clone())
                .or_else(|| env::var(var_name).ok())
                .unwrap_or(String::new())
        })
        .to_string()
}
