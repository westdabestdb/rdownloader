use bytes::Bytes;
use dirs::download_dir;
use dirs::home_dir;
use exitcode;
use loading::Loading;
use reqwest::{get, Response};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{fs::File, io::Write, path::PathBuf, vec};
#[derive(Serialize, Deserialize, Debug)]
struct RedditPost {
    data: serde_json::Value,
}

#[derive(Clone, Debug)]
pub struct VideoUrlRes {
    pub url: String,
    pub resolution: String,
}

pub struct Video {
    pub title: String,
    pub url_resolution: Vec<VideoUrlRes>,
    pub media_id: String,
}

const RESOLUTIONS: [&'static str; 9] = [
    "1080", "720", "480", "360", "240", "140", "2_4_M", "120", "1_2_M",
];

pub async fn get_video(url: String) -> Video {
    let loading = Loading::default();
    loading.text("Finding the video metadata...");
    let mut video_urls: Vec<VideoUrlRes> = vec![];
    let reddit_response: Vec<RedditPost> = get(format!("{}.json", url))
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let reddit_post = reddit_response[0].data.clone();

    let reddit_post_video_url: String = match reddit_post["children"][0]["data"]["secure_media"]
        ["reddit_video"]["fallback_url"]
        .clone()
    {
        serde_json::Value::String(s) => s,
        _ => {
            loading.fail("Only posts with videos are supported.");
            loading.end();
            std::process::exit(exitcode::DATAERR);
        }
    };

    let media_url: &str = reddit_post_video_url
        .split("https://v.redd.it/")
        .collect::<Vec<&str>>()[1];
    let media_id: &str = media_url.split("/").collect::<Vec<&str>>()[0];
    loading.success("Video metadata found.");
    loading.end();

    video_urls = get_video_with_resolutions(media_id, &mut video_urls).await;

    let mut video_title: String = reddit_post["children"][0]["data"]["title"].to_string();
    video_title = get_video_title(video_title);

    if video_urls.len() == 0 {
        loading.fail("No video found.");
        loading.end();
        std::process::exit(exitcode::DATAERR);
    }

    Video {
        title: video_title,
        url_resolution: video_urls,
        media_id: media_id.to_string(),
    }
}

async fn get_video_with_resolutions(
    media_id: &str,
    video_urls: &mut Vec<VideoUrlRes>,
) -> Vec<VideoUrlRes> {
    let loading = Loading::default();
    loading.text("Fetching video resolutions...");
    for resolution in RESOLUTIONS.iter() {
        let url: String = format!("https://v.redd.it/{}/DASH_{}.mp4", media_id, resolution);
        let response: Response = get(&url).await.unwrap();
        if response.status() == 200 {
            video_urls.push(VideoUrlRes {
                url,
                resolution: resolution.to_string(),
            });
        }
    }
    loading.success("Video resolutions fetched.");
    loading.end();
    return video_urls.to_vec();
}

fn get_video_title(title: String) -> String {
    return title.to_lowercase().replace(" ", "_").replace("\"", "");
}

pub async fn save_video(video: Video, selected_index: usize, save_location: String) {
    let loading = Loading::default();
    loading.text("Saving video...");
    let mut download_dir: PathBuf = download_dir().unwrap();
    let file_name: String = format!("{}_{}", "Rdownloader", video.title);
    if save_location != "" {
        download_dir = home_dir().unwrap().join(&save_location);
    }
    let file_path: String = format!("{}.mp4", file_name);
    let file_path_full: PathBuf = download_dir.join(file_path);
    let mut file: File = File::create(file_path_full).unwrap();
    let response = get(video.url_resolution[selected_index].url.clone())
        .await
        .unwrap()
        .bytes()
        .await;

    match response {
        Ok(s) => {
            // combine audio and video bytes
            file.write_all(&s).unwrap();
            if save_location == "" {
                loading.success(format!("Video saved to downloads directory"));
            } else {
                loading.success("Video saved in downloads folder");
            }
            loading.end();
            std::process::exit(exitcode::OK);
        }
        Err(_e) => {
            loading.fail("Unable to save video.");
            loading.end();
            std::process::exit(exitcode::DATAERR);
        }
    }
}
