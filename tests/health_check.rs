use std::net::TcpListener;

// `actix_rt::test` is the testing equivalent of `actix_web::main`. 
// It also spares you from having to specify the `#[test]` attribute. 
// 
// Use `cargo add actix-rt@2 --dev` to add `actix-rt` 
// under `[dev-dependencies]` in Cargo.toml 
//
// You can inspect what code gets generated using 
// `cargo expand test health_check` (<-name of the test file) 
#[actix_rt::test]
async fn health_check_works()  {
    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// No .await call, therefore no need to for spawn_app to be async now.
// We are also running tests, so it is not worth it to propagte the errors:
// if we fail to perform the required setup we can just panic and crash all the things.
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    //
    // New dev dependency: `cargo add tokio@1 --dev`
    let _ = tokio::spawn(server);

    // Return the application address to the caller
    format!("http://127.0.0.1:{}", port)
}
