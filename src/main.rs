/*
This actix Microservice has multiple routes:
1. `/`: return "Hello, find something to read!"
2. `/books`: return a random book name to the user
3. `/version`: return the version of the service
*/
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use find_books::random_book;
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, find something to read!")
}

#[get("/book")]
async fn books() -> impl Responder {
    println!("Explore this book: {}", random_book());
    HttpResponse::Ok().body(random_book())
}
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
