use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    // We have removed the hard-coded `8000` - it's now coming from our settings.
    let address = format!("127.0.0.1:{}", configuration.application_port);

    // Bubble up the io::Error if we failed to bind the address
    // otherwise call .await on our server
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
