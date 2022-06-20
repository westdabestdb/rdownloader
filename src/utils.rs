use std::collections::HashMap;

use crate::Regex;

pub fn process_url(url: &str) -> String {
    let mut url = url.to_string();
    if !url.contains("www.") {
        url = format!("www.{}", url);
    }
    if !url.contains("https://") {
        url = format!("https://{}", url);
    }
    url
}

pub fn is_url_valid(url: &str) -> bool {
    let url_regex = Regex::new(
        r"(?m)^(?:https?://)?(?:www\.)?reddit.com/r/[a-zA-Z0-9_]+/comments/[a-zA-Z0-9_]+/[a-zA-Z0-9_]+/?",
    )
    .unwrap();
    url_regex.captures(url).is_some()
}

pub fn parse_custom_location(mut reddit_url: String) -> HashMap<String, String> {
    let mut split_url = reddit_url.split("-p");
    let url_without_o = split_url.next().unwrap();
    let save_location = split_url.next().unwrap().to_string().remove(0).to_string();
    reddit_url = url_without_o.to_string();

    let mut custom_location = HashMap::new();
    custom_location.insert("reddit_url".to_string(), reddit_url.to_string());
    custom_location.insert("save_location".to_string(), save_location.to_string());
    custom_location
}
