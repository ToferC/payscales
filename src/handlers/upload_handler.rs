use actix_web::{post, Error, web};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

use std::io::Write;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "group_schema.graphql",
    query_path = "group_query.graphql",
    response_derives = "Debug"
)]
pub struct Query;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "group_schema.graphql",
    query_path = "group_query.graphql",
    response_derives = "Debug"
)]
pub struct AnniversaryQuery;

#[derive(Deserialize, Debug)]
pub struct Record {
    last_name: String,
    first_name: String,
    group: anniversary_query::GroupID, 
    level: i64, 
    anniversary_date: NaiveDate, 
    start_date: NaiveDate, 
    end_date: NaiveDate,
}

#[derive(Serialize, Deserialize)]
pub struct WBColumn {
    last_name: String,
    first_name:  String,
    group: String,
    level: i64,
    step: i64,
    anniversary_date: String,
    start_date:  String,
    end_date: String,
    work_hours: f64,
    work_days: f64,
    hourly_rate: f64,
    annual_rate: f64,
    salary: f64,
}

#[post("/")]
async fn upload_file(mut payload: Multipart) -> Result<NamedFile, Error> {

    let mut csv_iter = Vec::new();
    let mut data_vec = Vec::new();
    let mut wtr = csv::Writer::from_path("result.csv").unwrap();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let _filepath = format!("./tmp/{}", &filename);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create("tmp.csv"))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
     
    let mut rdr = csv::Reader::from_path("tmp.csv").unwrap();

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        csv_iter.push(record);
    };

    for r in csv_iter {

        let group_str: String;
        
        {
            group_str = format!("{:?}", &r.group);
        }

        let response_data = anniversary_pay_query(r.group, r.level, r.anniversary_date, r.start_date, r.end_date).await?;

        for period in response_data {
            
            let (step, start, end, work_hours, work_days, hourly_rate, annual_rate, salary) = period;

            data_vec.push(WBColumn {
                last_name: r.last_name.clone(),
                first_name:  r.first_name.clone(),
                group: group_str.clone(),
                level: r.level,
                anniversary_date: r.anniversary_date.to_string(),
                step: step,
                start_date:  start,
                end_date: end,
                work_hours: work_hours,
                work_days: work_days,
                hourly_rate: hourly_rate,
                annual_rate: annual_rate,
                salary: salary,
            })
        }
    }

    // Block for file operations
    let _r = web::block(move || {

        for e in &data_vec {
            wtr.serialize(e).unwrap();
        }
    
        wtr.flush()
    }).await?;

    Ok(NamedFile::open("result.csv")?)
}

async fn pay_query(
    identifier1: query::GroupID, 
    level: i64, 
    step: i64, 
    start_date: NaiveDate, 
    end_date: NaiveDate) -> Result<Vec<(String, String, f64, f64, f64, f64, f64)>, Error> {

    let request_body = Query::build_query(query::Variables {
        identifier1, 
        level, 
        step, 
        start_date, 
        end_date
    });

    // Async request
    let res = reqwest::Client::new()
        .post("https://gc-payscales.herokuapp.com/graphql")
        .json(&request_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let response_body: Response<query::ResponseData> = res;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: query::ResponseData = response_body.data.expect("missing response data");

    let pay_period = response_data.group.pay_at_level_and_step_between_dates.expect("Missing Pay Period");

    let mut response_vec = Vec::new();

    for period in pay_period {
        
        println!("Work Days: {:#?}", period.work_days);
        println!("Work Hours: {:#?}", period.work_hours);
        println!("Hourly Rate: {:#?}", period.hourly_rate);
        println!("Annual Rate: {:#?}", period.annual_rate);
    
        let salary_option = period.salary;
        let salary: f64;
    
        if let Some(s) = salary_option {
            salary = s;
        } else {
            salary = 0.0;
        }
    
        println!("Salary: {:#?}", salary);

        let start = period.start_date.format("%Y-%m-%d").to_string();
        let end = period.end_date.format("%Y-%m-%d").to_string();
    
        response_vec.push((start, end, period.work_hours, period.work_days, period.hourly_rate,
            period.annual_rate, salary));
    
        }
        
        Ok(response_vec)
}


async fn anniversary_pay_query(
    identifier1: anniversary_query::GroupID, 
    level: i64, 
    anniversary_date: NaiveDate, 
    start_date: NaiveDate, 
    end_date: NaiveDate) -> Result<Vec<(i64, String, String, f64, f64, f64, f64, f64)>, Error> {

    let request_body = AnniversaryQuery::build_query(anniversary_query::Variables {
        identifier1, 
        level, 
        anniversary_date, 
        start_date, 
        end_date
    });

    // Async request
    let res = reqwest::Client::new()
        .post("https://gc-payscales.herokuapp.com/graphql")
        .json(&request_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let response_body: Response<anniversary_query::ResponseData> = res;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    }

    let response_data: anniversary_query::ResponseData = response_body.data.expect("missing response data");

    let pay_period = response_data.group.pay_at_level_by_anniversary_date_between_dates.expect("Missing Pay Period");

    let mut response_vec = Vec::new();

    for period in pay_period {
        
        println!("Work Days: {:#?}", period.work_days);
        println!("Work Hours: {:#?}", period.work_hours);
        println!("Hourly Rate: {:#?}", period.hourly_rate);
        println!("Annual Rate: {:#?}", period.annual_rate);
    
        let salary_option = period.salary;
        let salary: f64;
    
        if let Some(s) = salary_option {
            salary = s;
        } else {
            salary = 0.0;
        }
    
        println!("Salary: {:#?}", salary);

        let start = period.start_date.format("%Y-%m-%d").to_string();
        let end = period.end_date.format("%Y-%m-%d").to_string();
    
        response_vec.push((period.step, start, end, period.work_hours, period.work_days, period.hourly_rate,
            period.annual_rate, salary));
    
        }
        
        Ok(response_vec)
}