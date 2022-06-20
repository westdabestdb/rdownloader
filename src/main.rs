use crate::constants::*;
use crate::prompt::type_reddit_url;
use crate::reddit::get_video_audio;
use crate::reddit::save_video;
use crate::utils::is_url_valid;
use crate::utils::parse_custom_location;
use crate::utils::process_url;
use crate::{prompt::select_reddit_video_resolution, reddit::get_video};
use colored::*;
use loading::Loading;
use regex::Regex;
mod constants;
mod prompt;
mod reddit;
mod utils;

#[tokio::main]
async fn main() {
    println!("{}", RDOWNLOADER_ASCII_TEXT.green());
    println!("{}", WELCOME_MESSAGE.cyan().bold());

    let mut reddit_url = type_reddit_url();
    let mut save_location = String::new();
    if reddit_url.contains("-p") {
        let custom_location = parse_custom_location(reddit_url);
        save_location = custom_location["save_location"].to_string();
        reddit_url = custom_location["reddit_url"].to_string();
    }

    let loading = Loading::default();
    loading.text("Checking the url...");
    let is_valid: bool = is_url_valid(&reddit_url);
    if !is_valid {
        loading.fail(format!("{} is not a Reddit post url", reddit_url));
        loading.end();
        std::process::exit(exitcode::DATAERR);
    } else {
        loading.success("Url is valid.");
        loading.end();
    }

    reddit_url = process_url(&reddit_url);

    let video_data = get_video(reddit_url).await;
    let mut resolutions: Vec<String> = Vec::new();
    for ur in video_data.url_resolution.iter() {
        resolutions.push(ur.resolution.to_string());
    }

    let selected_resolution_index = select_reddit_video_resolution(resolutions);
    save_video(video_data, selected_resolution_index, save_location).await;
}
