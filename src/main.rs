extern crate actix_web;
use std::fs::File;
use std::io::prelude::*;
use actix_web::{server, App, HttpRequest, HttpResponse, http::ContentEncoding};

fn index(_req: HttpRequest) -> HttpResponse {
    let filename = "index.html";
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html")
        .body(contents)
}

fn main() {
    let addr = "127.0.0.1:8000";
    println!("Server running at: http://{}", addr);
    server::new(|| App::new().resource("/", |r| r.f(index)))
        .bind(addr)
        .unwrap()
        .run();
}
