use zero2prod::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Bubble up the io::Error if we failed to bind the address
    // otherwise call .await on our server
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    run(listener)?.await
}