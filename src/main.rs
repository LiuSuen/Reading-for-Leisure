/*
This actix Microservice has multiple routes:
1. `/`: return "Hello, find something to read!"
2. `/book`: return a random book name to the user
3. `/version`: return the version of the service
*/
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use find_books::random_book; //import random_book function from lib.rs
//return the welcome
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, let's find something to read!")
}
//return a book name
#[get("/book")]
async fn books() -> impl Responder {
    println!("Explore this book: {}", random_book());
    HttpResponse::Ok().body(random_book())
}
//return the version of the service
#[get("/version")]
async fn version() -> impl Responder {
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the service.");
    HttpServer::new(|| App::new().service(hello).service(books).service(version))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
