#[macro_use]

use std::sync::Arc;

use actix_web::{web, get, post, HttpResponse, Responder, Error};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

use crate::graphql_schema::{Schema};

// Graphql

#[post("/graphql")]
pub async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let group = web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .await?;


    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(group))
}

pub async fn graphiql() -> HttpResponse {
    let html = graphiql_source("/graphiql");
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)

}