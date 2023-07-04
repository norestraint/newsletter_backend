use std::net::TcpListener;

use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Launching app at: 127.0.0.1:{}", &port);
    run(listener)?.await
}
