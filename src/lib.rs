mod parsing_core;
mod parsing_params;

#[cfg(test)]
mod tests;

use linkify::{LinkFinder, LinkKind};
use parsing_core::Parser;
use std::collections::HashSet;
use url::Url;

/// Takes any string as input, parses URLs, returns either `None` if no tracking tokens
/// were found. Otherwise returns `Some(Vec<String>)` of all sanitized URLs
pub fn clean_urls_from_any_text(input: &String) -> Option<Vec<String>> {
    let parser = Parser::new();
    let cleaned_urls = parser.parse_any_text(input);
    cleaned_urls
}

/// Parses any text and sanitizes URLs containing tracking tokens in place
pub fn replace_urls_in_place(input: &mut String) -> Option<&mut String> {
    let parser = Parser::new();
    let changes = parser.sanitize_in_place(input);
    if changes.is_some() {
        Some(input)
    } else {
        None
    }
}
