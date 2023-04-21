use zero2prod::run;
use std::net::TcpListener;

// We can't use the entrypoint for the binary to be an async function - so how does the main async function work here?
// The #[actix_web::main] part is a macro which takes in a "stream of symbols" (ie, in this case the main function) and outputs a new stream of symbols. It basically rewrites the function before it's passed to the compiler
// We can view the function it rewrites by running "cargo expand".
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    run(listener)?.await
}
