use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};
use std::env;

mod handlers;

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
    let environment = env::var("ENVIRONMENT");

    let environment = match environment {
        Ok(v) => v,
        Err(_) => String::from("test"),
    };

    let (host, port) = if environment == "production" {
        (env::var("HOST").unwrap(), env::var("PORT").unwrap())
    } else {
        (String::from("127.0.0.1"), String::from("8088"))
    };

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
            .service(handlers::api_base)
            .service(handlers::api_group)
            .service(handlers::api_group_level)
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
