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

    HttpServer::new(|| {
        let cors = Cors::default()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .route("/{filename:.*}", web::get().to(index))
    })
    .bind("192.168.178.103:8883")?
    .run()
    .await
}