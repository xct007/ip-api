use env_logger;
use actix_web::{web, App, HttpServer};
use ip_address::handlers::Handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let port: String = String::from("8080"); // Replace with your desired port number

    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(Handler::ip))
            .route("/health", web::get().to(Handler::health))
    })
    .bind(format!("127.0.0.1:{}", port))?;

    let server: actix_web::dev::Server = server.run();

    server.await
}
