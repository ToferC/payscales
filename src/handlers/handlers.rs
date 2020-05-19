use actix_web::{web, get, HttpResponse, Responder};

/// API outline
/// /api -> list of groups
/// /api/{group} -> details on the group in JSON
/// /api/{group}/{level} -> pay for group & level at today's rates and maximum step
/// /api/{group}/{level}/{step} -> pay for group, level and step at today's rates
/// /api/{group}/{level}/{step}/{date} -> pay for group, level and step at date rates
/// /api/{group}/{level}/{step}/{date}/{period in days or hours -- 5d, 37.5h} -> pay for that period

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
    <h1>Welcome to GC-PayScales</h1>
    <p>This application is an API for Government of Canada pay/salary scales for classifications and levels. The intent is to have a central, easy to access and use resource for accurate pay information. This will reduce duplication and potential errors across organizations and serve as an example of how APIs can support day-to-day business in government.</p>
    
    <p>This is also a learning project in Rust and my first foray into GraphQL development.</p>
    
    <p>You can explore and experiment with the API here: <a href="https://gc-payscales.herokuapp.com/playground">https://gc-payscales.herokuapp.com/playground</a></p>
    
    <p>A sample query looks like this:
    {
        group(identifier:	EC) {
          payscaleForLevel(level: 3) {
            name
            level
            steps
            currentRatesOfPay{
              salary(step: 3)
            }
            payOnDateForLevelAndStep(date: "2020-06-23"){
              inForce
              salary(step: 3)
            }
          }
        }
      }</p>

    <p>Please note that this work is a learning project, may contain errors and should not be used to make pay-related decisions.</p>
    <p>The PayScraper project is available on GitHub under an MIT licence here: <a href="https://github.com/ToferC/payscraper">https://github.com/ToferC/payscraper</a></p>
    <p>The API is available here under the same licence: <a href="https://github.com/ToferC/payscales">https://github.com/ToferC/payscales</a></p>

    <p>Developed by ToferC 2020</p>


    
    "#)
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





