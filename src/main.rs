use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[get("/again")]
async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello World again")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
