use actix_files::Files;
use actix_web::{middleware, App, HttpServer};

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
            .service(Files::new("/", "./static/root/").index_file("index.html"))
    })
    .bind("192.168.178.33:8765")?
    .run()
    .await
}