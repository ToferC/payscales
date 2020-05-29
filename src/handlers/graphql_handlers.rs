use std::sync::Arc;

use actix_web::{web, post, HttpResponse, Error};
use juniper::http::graphiql::graphiql_source;
use juniper::http::{GraphQLRequest};
use juniper::http::playground::playground_source;

use crate::graphql_schema::{Schema};
use crate::DataBase;

// Graphql

#[post("/graphql")]
pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {

    let ctx = DataBase::new();

    let groups = web::block(move || {
        let res = data.execute(&st, &ctx);
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(groups))
}

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}

pub async fn playground_handler() -> HttpResponse {
    let html = playground_source("/graphql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}