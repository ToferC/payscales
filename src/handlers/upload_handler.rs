use actix_web::{post, Error, web};
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "group_schema.graphql",
    query_path = "group_query.graphql",
    response_derives = "Debug"
)]
pub struct Query;

#[derive(Deserialize, Debug)]
pub struct Record {
    last_name: String,
    first_name: String,
    group: query::GroupID, 
    level: i64, 
    step: i64, 
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
    let mut data = bytes::Bytes::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let _filepath = format!("./tmp/{}", &filename);

        while let Some(chunk) = field.next().await {
            data = chunk.unwrap();
        }
    }
    
    let mut rdr = csv::Reader::from_reader(data.as_ref());

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        csv_iter.push(record);
    }

    for r in csv_iter {

        let group_str: String;
        
        {
            group_str = format!("{:?}", &r.group);
        }

        let response_data = pay_query(r.group, r.level, r.step, r.start_date, r.end_date).await?;

        for period in response_data {
            
            let (start, end, work_hours, work_days, hourly_rate, annual_rate, salary) = period;

            data_vec.push(WBColumn {
                last_name: r.last_name.clone(),
                first_name:  r.first_name.clone(),
                group: group_str.clone(),
                level: r.level,
                step: r.step,
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

    for e in &data_vec {
        wtr.serialize(e).unwrap();
    }

    let _r = web::block(move ||
        wtr.flush()
    );

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

    let client = reqwest::Client::new();
    let mut res = client.post("https://gc-payscales.herokuapp.com/graphql").json(&request_body).send().expect("Error sending query");
    let response_body: Response<query::ResponseData> = res.json().expect("Failed to receive response");

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