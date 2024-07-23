# untrack crate
This crate is intended to remove tracking tokens from URLs contained in any text input. The crate can either substitute the URLs in place or return a Vec of cleaned urls to a given text input.

## Usecase
Removing tracking tokens can improve the privacy of the user sharing the link containing the tokens and the consumer opening the shared link.

## Supported tracking tokens
The crate currently supports Twitter, X, YouTube, Substack and Spotify tracking tokens. The exact tokens can be found in ```src/parsing_params.rs```.

## Example usage

### fn clean_urls_from_any_text(input: &String) -> Option<Vec<String>>
```
let input = 
String::from("Twitter link: https://twitter.com/user/status/123?utm_source=test&s=1234");

let expected = vec!["https://twitter.com/user/status/123".to_string()];

assert_eq!(clean_urls_from_any_text(&input), Some(expected));
```

### fn replace_urls_in_place(input: &mut String) -> Option<&mut String>
```
let mut input = String::from("Multiple URLs: https://twitter.com/user/status/123?s=12 and https://www.youtube.com/watch?v=abc&feature=share");

let result = replace_urls_in_place(&mut input);

assert!(result.is_some());

assert_eq!(input, "Multiple URLs: https://twitter.com/user/status/123 and https://www.youtube.com/watch?v=abc");
```