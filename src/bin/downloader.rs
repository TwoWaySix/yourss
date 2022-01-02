use std::process::Command;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::client::Client;

#[get("/download/from/youtube/{video_id}")]
async fn index(web::Path(video_id): web::Path<String>) -> impl Responder {
    let url = format!("https://www.youtube.com/watch?v={}", video_id);

    println!("Trying to download audio from following URL: {}", url);
    let stdout = download_from_url(&url);
    println!("Download finished or failed.");

    // Updating the feed
    let feedbuilder_ip = match std::env::var_os("YOURSS_FEEDBUILDER") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$YOURSS_FEEDBUILDER is not set. Please set it to the ip address and port.")
    };
    let req = format!("http://{}/update/feed", feedbuilder_ip);
    println!("Rebuilding feed by calling: {}", req);
    let mut client = Client::default();
    let payload = client.get(req)
            .send()
            .await
            .unwrap();

    format!("Download finished from : {} \n {}", url, stdout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address = match std::env::var_os("YOURSS_DOWNLOADER") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$YOURSS_DOWNLOADER is not set. Please set it to the ip address and port.")
    };

    println!("Starting YouRSS Downloader at {}.", ip_address);

    HttpServer::new(|| App::new().service(index))
        .bind(ip_address)?
        .run()
        .await
}

fn download_from_url(url: &str) -> String {
    let output = Command::new("youtube-dl")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg("./static/mp3/%(title)s.$(ext)s")
        .arg(url)
        .output()
        .expect("Failed to download file.");

    let stdout = output.stdout.as_slice();
    format!("{:?}", stdout)
}
