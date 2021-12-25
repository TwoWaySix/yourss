use std::fs;
use std::fs::File;
use actix_web::client::Client;
use actix_web::dev::JsonBody;
use actix_web::web::{Json, Payload};
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

    let rss_feed = create_rss_feed(&host_url, &feed_template_path);

    let mut client = Client::default();

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

    println!("\n\nResponse: {:?}", payload);

    let obj = serde_json::from_slice::<FileList>(&payload).unwrap();

    println!("Response: {:?}", obj);


    // TODO: Replace previous feed.xml
}

fn create_enclosed_tag(tag: &str, child: &str) -> String {
    format!("<{}>{}</{}>", tag, child, tag)
}

fn create_rss_feed(host_url: &str, feed_template_path: &str) -> String {
    let feed_path = format!("{}/rss/feed.xml", host_url);
    let logo_path = format!("{}/images/logo.png", host_url);

    let template = fs::read_to_string(feed_template_path)
            .expect("Something went wrong reading the file");

    let items = create_feed_items(&host_url);

    let feed = template
        .replace("{host_url}", &host_url)
        .replace("{logo_url}", &logo_path)
        .replace("{items}", &items);

    println!("{}", feed);

    feed
}

fn create_feed_items(host_url: &str) -> String {
    let mut items = Vec::new();
    let mut episode_urls = Vec::new();

    // TODO: Read feed items from mp3 folder
    let episode_url = format!("{}/mp3/Riesenkomet.mp3", host_url);

    episode_urls.push(episode_url);

    for url in episode_urls {
        // TODO: Calculate duration
        let duration = "734.7461224489796".to_string();
        items.push(create_feed_item(&url, &duration));
    }
    items.join("\n")
}

fn create_feed_item(episode_url: &str, duration: &str) -> String {
    let item = format!(
        "<item>
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
         title="Title",
         episode_url=episode_url,
         image_url=episode_url.replace("mp3", "jpg"),
         duration=duration
    );
    item
}
