use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use chrono::prelude::*;

use crate::models::{Group, RateOfPay};

pub fn load_group_data() -> Result<Vec<Group>, Box<dyn Error>> {
    let file = File::open("./data/groups_data.json").expect("could not open file");
    let reader = BufReader::new(file);
    
    let group_data: Vec<Group> = serde_json::from_reader(reader).unwrap();

    Ok(group_data)
}

pub fn convert_string_to_naive_date(target: &String) -> NaiveDate {
    
    let naive_date = NaiveDate::parse_from_str(
        target.as_str(),
        "%Y-%m-%d").unwrap();

    naive_date
}

pub fn round_to_2_decimal_points(float: f64) -> f64 {
    (float * 100.0).round() / 100.0
}

/// Accepts a Vec<RateOfPay> and a YYYY-MM-DD string and returns the rate_of_pay in effect for the date provided, past, present or future.
pub fn check_active_pay_rate(rates: &Vec<RateOfPay>, target_date: NaiveDate) -> &RateOfPay {

    let mut target = 0;
    let end_index = rates.len() - 1;
    
    for (i,_rate_of_pay) in rates.iter().enumerate() {

        if i < end_index {
            // set start_date forrate_of_pay 
            let start_date = convert_string_to_naive_date(&rates[i].date_time);

            // get the end date forrate_of_pay
            let end_date = convert_string_to_naive_date(&rates[i+1].date_time);

            // Check to see if today's date is withing therate_of_pay start and end dates

            if target_date > start_date && target_date <= end_date {
                // set target to current index
                target = i;
                break
            }
        
        } else {
            if target_date < convert_string_to_naive_date(&rates[0].date_time) {
                // target date is before first active pay rate
                // We will use the first existing pay rate to calculate
                target = 0
            } else {
                // Current date isn't within an in force rate_of_pay
                // So we should use the lastrate_of_pay available
                target = end_index;
            }

        }
    }
    
    // returnrate_of_pay for this date
    &rates[target]
}