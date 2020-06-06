use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use std::cmp;

use chrono::prelude::*;
use chrono::Duration;

use crate::models::{Group, PayScale, RateOfPay, ActiveRateOfPay};

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

/// Accepts a Vec<RateOfPay> and a YYYY-MM-DD string and returns the rate_of_pay in effect for the date provided, past, present or future.
pub fn return_active_pay_for_period(
    payscale: &PayScale,
    steps: Vec<(NaiveDate, NaiveDate)>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Vec<ActiveRateOfPay> {

    let rates = &payscale.rates_of_pay;

    let mut target = 0;
    let mut step = 0;
    let mut anniversary_date: NaiveDate = NaiveDate::from_ymd(1960, 01, 01);
    let mut rate_end: NaiveDate = NaiveDate::from_ymd(1960, 01, 01);

    let end_index = rates.len() - 1;
    let max_step = payscale.steps as usize - 1;

    let mut active_rates_of_pay = Vec::new();

    // find step index at start of period
    for (i, (s, e)) in steps.iter().enumerate() {
        if start_date > *s && start_date < *e {
            step = i;
            anniversary_date = *s
        }
    };

    // Do we just add new

    // return the step with a max derived from the payscale
    let cap_step = {|s| cmp::min(s, max_step)};

    // return the latter of two NaiveDates
    let later_date = {|d1: NaiveDate, d2: NaiveDate| {
        if d1 > d2 {
            d1
        } else {
            d2
        }
    }};

    let earlier_date = {|d1: NaiveDate, d2: NaiveDate| {
        if d1 < d2 {
            d1
        } else {
            d2
        }
    }};

    let first_in_force = convert_string_to_naive_date(&rates[0].date_time);

    // get potential period that starts before in force for payscale
    if start_date < first_in_force && end_date < first_in_force {

        let mut sd = start_date;
        
        while sd < end_date {
            // loop through steps
            for st in &steps {
                if st.0 > start_date && st.0 < first_in_force {

                    // returnrate_of_pay for this date and starting step
                    let salary = &rates[target].salary.get(cap_step(step));
                    let sal = match salary {
                        Some(s) => **s,
                        _ => 0,
                    };

                    let a = ActiveRateOfPay {
                        start_date: sd,
                        end_date: earlier_date(st.1, end_date),
                        step: cap_step(step) as i32 + 1,
                        salary: sal,
                    };

                    active_rates_of_pay.push(a);
                    sd = earlier_date(st.1, end_date);
                    step += 1;
                };
            };
        };
    }
    
    active_rates_of_pay
}