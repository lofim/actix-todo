extern crate actix_web;
use actix_web::{server, App, HttpRequest};

fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

fn main() {
    let url = "127.0.0.1:8088";
    println!("Running server on {}", url);
    
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(url)
        .unwrap()
        .run();
}
