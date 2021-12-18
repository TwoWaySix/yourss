use std::fs;
use actix_files::Files;
use actix_web::{middleware, App, HttpServer, web, get, HttpRequest};
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("/images", "static/images/").show_files_listing())
            .service(Files::new("/rss", "static/rss/").show_files_listing())
            .service(Files::new("/mp3", "static/mp3/").show_files_listing())
            .service(mp3_filenames)
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind("192.168.178.10:8765")?
    .run()
    .await
}

#[derive(Serialize)]
struct FileList {
    file_names: Vec<String>
}

#[get("/mp3_files")]
async fn mp3_filenames(req: HttpRequest) -> String {
    let mut file_names = Vec::new();

    for entry in fs::read_dir("./static/mp3").unwrap() {
        let dir = entry.unwrap();
        let f = format!("{}", dir.path().file_name().unwrap().to_str().unwrap());
        file_names.push(f);
    }
    println!("REQ: {:?}", req);
    format!("{:?}", file_names.join(";"))
}