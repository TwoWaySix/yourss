use actix_files::NamedFile;
use actix_web::{http, HttpRequest, Result};
use std::path::PathBuf;
use actix_cors::Cors;

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let path = PathBuf::from("static").join(path);
    println!("{:?}", path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    let ip_address = match std::env::var_os("YOURSS_FRONTEND") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$YOURSS_FRONTEND is not set")
    };

    // TODO: Get yourss-downloader ip from env and writing it into the index.js

    println!("Starting YouRSS Frontend at {}.", ip_address);
    HttpServer::new(|| {
        let cors = Cors::default()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind(ip_address)?
    .run()
    .await
}