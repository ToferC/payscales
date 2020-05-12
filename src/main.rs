
#[macro_use]
extern crate juniper;

extern crate diesel;

use std::env;
use std::io;

use actix_web::{App, HttpServer, web, middleware};


mod graphql_schema;

use crate::graphql_schema::{create_schema, Schema};


use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod errors;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    // Set environment variables
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

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
    
    /*
    // create database connection pool
    // Diesel Postgres
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    */ 

    // Create Juniper Schema
    let schema = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .wrap(middleware::Logger::default())
            .service(handlers::index)
            .service(handlers::api_base)
            .service(handlers::api_group)
            .service(handlers::api_group_level)
            .service(handlers::graphql)
            .service(web::resource("/graphiql").route(web::get().to(handlers::graphiql)))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
