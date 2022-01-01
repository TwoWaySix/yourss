use std::process::Command;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/download/from/youtube/{url}")]
async fn index(web::Path(url): web::Path<String>) -> impl Responder {
    let url_full = format!("https://www.youtube.com/watch?v={}", url);

    println!("YouRSS - Downloader");
    println!("Trying to download audio from following URL: {}", url_full);
    let stdout = download_from_url(&url_full);
    println!("Download finished or failed.");

    format!("Download finished from : {} \n {}", url_full, stdout)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address = match std::env::var_os("YOURSS_DOWNLOADER") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$YOURSS_DOWNLOADER is not set")
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
