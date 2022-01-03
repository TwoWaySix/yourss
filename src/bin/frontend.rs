use std::fs::File;
use std::io::prelude::*;
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
    let ip_address_downloader = match std::env::var_os("YOURSS_DOWNLOADER") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$YOURSS_DOWNLOADER is not set")
    };

    change_ip_in_js_files(&ip_address_downloader);

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

fn change_ip_in_js_files(ip_address: &str) -> std::io::Result<()> {
    // Read javascript file
    let mut file = File::open("./static/js/index.js")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Changing ip address
    contents.replace("localhost:8882", ip_address);

    // Writing new javascript file
    let mut file = File::create("./static/js/index.js")?;
    file.write_all(contents.as_bytes());

    Ok(())
}