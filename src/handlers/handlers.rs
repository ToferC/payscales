use actix_web::{web, get, HttpResponse, HttpRequest, Responder};
use crate::AppData;
use tera::{Context};

/// API outline
/// /api -> list of groups
/// /api/{group} -> details on the group in JSON
/// /api/{group}/{level} -> pay for group & level at today's rates and maximum step
/// /api/{group}/{level}/{step} -> pay for group, level and step at today's rates
/// /api/{group}/{level}/{step}/{date} -> pay for group, level and step at date rates
/// /api/{group}/{level}/{step}/{date}/{period in days or hours -- 5d, 37.5h} -> pay for that period

#[get("/")]
async fn index(data: web::Data<AppData>, _req:HttpRequest) -> impl Responder {
    let ctx = Context::new(); 
    let rendered = data.tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/api")]
pub async fn api_base() -> impl Responder {
    HttpResponse::Ok().body("Placeholder for API for Government of Canada payscales")
}

#[get("/api/group/{group}")]
pub async fn api_group(info: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Return info for group: {}", info))
}

#[get("/api/group/{group}/{level}")]
pub async fn api_group_level(info: web::Path<(String, usize)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Return pay for {} group and {} level at today's date", info.0, info.1))
}

#[get("/api/{group}/{level}/{step}")]
pub async fn api_group_level_step(info: web::Path<(String, usize, usize)>) -> impl Responder {
    HttpResponse::Ok().body(format!(
        "Return pay for {} group, {} level & {} step at today's date",
        info.0, info.1, info.2)
    )
}

#[get("/api/{group}/{level}/{date}")]
pub async fn api_group_level_date(_info: web::Path<(String, usize, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Return pay for group and level at today's date"))
}





