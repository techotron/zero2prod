use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    // Looking at the docs, you can see that Responder calls self.finish() so we don't need to do that here with HttpResponse::Ok().finish()
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
    })
    .listen(listener)?
    .run();
    // No .await here!
    Ok(server)
}



// Old code which used an async function for the entrypoint:
// pub async fn run() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }
