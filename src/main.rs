use actix_web::{web, get, App, HttpResponse, HttpServer, Responder};

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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(index2)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
