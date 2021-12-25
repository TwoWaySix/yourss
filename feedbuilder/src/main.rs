use std::fs;
use std::fs::File;
use std::io::prelude::*;

use std::path::PathBuf;
use actix_web::client::Client;
use actix_web::dev::JsonBody;
use actix_web::web::{Json, Path, Payload};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct FileList {
    file_names: Vec<String>
}

#[actix_web::main]
async fn main() {
    let feed_template_path = "./templates/feed_template.xml".to_string();
    let host_url = "http://192.168.178.103:8765".to_string();

    let mut client = Client::default();

    // TODO: Do proper error handling
    let payload =
        client
            .get(format!("{}/api/mp3", host_url))
            .send()
            .await
            .unwrap()
            .body()
            .limit(20_000_000)  // sets max allowable payload size
            .await
            .unwrap();
    let file_list = serde_json::from_slice::<FileList>(&payload).unwrap();

    println!("\n\nResponse: {:?}", payload);
    println!("Response: {:?}", file_list);

    let rss_feed = create_rss_feed(&host_url, &feed_template_path, &file_list);

    let path = PathBuf::from("./static/rss/feed.xml");
    write_feed(&path, &rss_feed);
}

fn create_enclosed_tag(tag: &str, child: &str) -> String {
    format!("<{}>{}</{}>", tag, child, tag)
}

fn create_rss_feed(host_url: &str, feed_template_path: &str, file_list: &FileList) -> String {
    let feed_path = format!("{}/rss/feed.xml", host_url);
    let logo_path = format!("{}/images/logo.png", host_url);

    let template = fs::read_to_string(feed_template_path)
            .expect("Something went wrong reading the file");

    let items = create_feed_items(&host_url, file_list);

    let feed = template
        .replace("{host_url}", &host_url)
        .replace("{logo_url}", &logo_path)
        .replace("{items}", &items);

    println!("{}", feed);

    feed
}

fn create_feed_items(host_url: &str, file_list: &FileList) -> String {
    let mut items = Vec::new();

    for file_name in &file_list.file_names {
        // TODO: Calculate duration
        let duration = "734.7461224489796".to_string();
        items.push(create_feed_item(&host_url, &file_name, &duration));
    }
    items.join("\n")
}

fn create_feed_item(host_url: &str, file_name: &str, duration: &str) -> String {
    let episode_url = format!("{}/mp3/{}", host_url, file_name);

    let item = format!(
        "        <item>
            <title>
                {title}
            </title>
            <itunes:title>
                {title}
            </itunes:title>
            <itunes:author>
                PODCAST CREATOR
            </itunes:author>
            <itunes:subtitle>
                EPISODE SUBTITLE
            </itunes:subtitle>
            <itunes:summary>
                ITUNES SUMMARY
            </itunes:summary>
            <description>
                DESCRIPTION
            </description>
            <itunes:image href=\"{image_url}\"/>
            <enclosure
                url=\"{episode_url}\"
                length=\"{duration}\"
                type=\"audio/mpeg\"
            />
            <itunes:duration>
                {duration}
            </itunes:duration>
            <itunes:season>
                SEASON #
            </itunes:season>
            <itunes:episode>
                EPISODE #
            </itunes:episode>
            <itunes:episodeType>
                full
            </itunes:episodeType>
            <guid isPermaLink=\"false\">
                {episode_url}
            </guid>
            <pubDate>
                EPISODE PUBLISH DATE
            </pubDate>
            <itunes:explicit>
                NO
            </itunes:explicit>
        </item>",
         title=file_name,
         episode_url=episode_url,
         image_url=episode_url.replace("mp3", "jpg"),
         duration=duration
    );
    item
}

fn write_feed(path: &PathBuf, feed: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(feed.as_bytes())?;
    Ok(())
}