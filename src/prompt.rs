use dialoguer::{theme::ColorfulTheme, Input, Select};

pub fn select_reddit_video_resolution(resolutions: Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the video resolution")
        .items(&resolutions[..])
        .interact()
        .unwrap()
}

pub fn type_reddit_url() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Paste the Reddit post URL. (Optional -p to output directory)")
        .interact()
        .unwrap()
}
