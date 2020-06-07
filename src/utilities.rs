use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use std::cmp;

use chrono::prelude::*;

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
pub fn return_active_rate_index(rates: &Vec<RateOfPay>, target_date: NaiveDate) -> usize {

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
    target
}

/// Accepts a PayScale, Vec<(NaiveDate, NaiveDate)> for pay steps
/// and YYYY-MM-DD strings for start and end date
/// rRturns a vec of active_rate_of_pay for the date range provided, past, present or future.
pub fn return_active_pay_for_period(
    payscale: &PayScale,
    steps: Vec<(NaiveDate, NaiveDate)>,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Vec<ActiveRateOfPay> {

    let rates = &payscale.rates_of_pay;

    let mut step = 0;

    let max_step = payscale.steps as usize - 1;
    let max_rate = payscale.rates_of_pay.len() - 1;

    let mut active_rates_of_pay = Vec::new();

    // find step index at start of period
    for (i, (s, e)) in steps.iter().enumerate() {
        if start_date > *s && start_date < *e {
            step = i;
        }
    };

    // Do we just add new

    // return the step with a max derived from the payscale
    let cap_step = {|s| cmp::min(s, max_step)};
    let cap_rate = {|r| cmp::min(r, max_rate)};

    let earlier_date = {|d1: NaiveDate, d2: NaiveDate| {
        if d1 < d2 {
            d1
        } else {
            d2
        }
    }};

    // period falls within in_force pay_scale
    let mut sd = start_date;
    let mut rate_index = return_active_rate_index(rates, sd);
    
    while sd < end_date {
        // loop through steps

        // returnrate_of_pay for this date and starting step
        let rp = &rates[cap_rate(rate_index)];
        let salary = rp.salary.get(cap_step(step));

        // Missing the first pay_rate

        let next_rate_index = {
            if rate_index >= max_rate {
                max_rate
            } else {
                rate_index + 1
            }
        };

        let next_in_force_date = convert_string_to_naive_date(&rates[next_rate_index].date_time);

        let sal = match salary {
            Some(s) => *s,
            _ => 0,
        };

        // st start == anniversary date in force
        // st end == anniversary date ends
        // in_force_date == new pay_scale comes in force
        // sd == the earlir of the pay scale start in a vec of years
        // or the date the current payscale came in force

        // Calculate whether to end next period with a step date or in_force_date
        let mut period_end = {
            if next_rate_index == &rates.len() - 1 {
                steps[step].1
            } else {
                if steps[step].1.signed_duration_since(sd) < next_in_force_date.signed_duration_since(sd) {
                    steps[step].1
                } else {
                    next_in_force_date
                }
            }
        };

        period_end = earlier_date(period_end, end_date);

        // Create struct
        let a = ActiveRateOfPay {
            start_date: sd,
            end_date: period_end,
            step: cap_step(step) as i32 + 1,
            salary: sal,
        };

        // Add to vec
        active_rates_of_pay.push(a);

        // set the new sd for loop
        sd = period_end;

        // catch last loop
        if sd >= end_date {
            break
        }

        // if the sd crosses a step, increment the step
        if sd >= steps[step].1 {
            step += 1;
        }

        if sd >= convert_string_to_naive_date(&rates[next_rate_index].date_time) {
            rate_index += 1;
        }        
    };
    
    active_rates_of_pay
}