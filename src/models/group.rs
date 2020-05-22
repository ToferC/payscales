use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;
use crate::utilities::{
    convert_string_to_naive_date,
    check_active_pay_rate,
    round_to_2_decimal_points
};

use crate::models::{PayScale, RateOfPay};
use super::pay_period::PayPeriod;
use super::enums::GroupID;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct Group {
    pub name: String,
    /// The two-letter identifier for the group
    pub identifier: GroupID,
    /// The URL for the collective agreement
    pub url: String,
    /// Vector of payscales for the group
    pub pay_scales: Vec<PayScale>,
    // Date scraped
    pub date_scraped: String,
    // Irregular format
    pub irregular_format: bool,
}

#[juniper::object(Context = DataBase)]
/// A pay group as defined by a collective agreement
impl Group {
    /// The Group's name as per the collective agreement.
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// The two-letter identifier for the group as an enum.
    pub fn identifier(&self) -> &GroupID {
        &self.identifier
    }
    /// The URL for the collective agreement.
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
    /// Vector of payscales for the group.
    pub fn payscales(&self) -> &Vec<PayScale> {
        &self.pay_scales
    }
    /// Date the collective agreement was scraped in YYYY-MM-DD.
    pub fn date_scraped(&self) -> NaiveDate {
        convert_string_to_naive_date(&self.date_scraped)
    }
    /// Returns a payscale for a specific level within the group.
    pub fn payscale_for_level(&self, level: i32) -> Option<&PayScale> {
        self.pay_scales.iter().find(|p| p.level == level)
    }

    pub fn pay_at_level_and_step_between_dates(&self, level: i32, step: i32, start_date: NaiveDate, end_date: NaiveDate) -> Option<Vec<PayPeriod>> {
        let payscale = self.pay_scales.iter().find(|p| p.level == level);

        let payscale = match payscale {
            Some(p) => p,
            None => return None
        };

        // Check for crossing Rates of pay based on dates and add each date to vec
        let mut relevant_rates_of_pay: Vec<&RateOfPay> = Vec::new();

        // Find starting rate of pay
        let mut start_pay_rate = check_active_pay_rate(&payscale.rates_of_pay, start_date).clone();
        
        // Alter date_time to starting date
        start_pay_rate.date_time = start_date.format("%Y-%m-%d").to_string();
        
        // Add our start_date to vec
        relevant_rates_of_pay.push(&start_pay_rate);

        for rp in &payscale.rates_of_pay {
            // get all payscale rates of pay in_force dates
            let target_date = convert_string_to_naive_date(&rp.date_time);
            
            // check if rate of pay active between dates and if so, add to our vec
            if target_date > start_date && target_date <= end_date {
                relevant_rates_of_pay.push(rp);
            }
        };

        // Find ending rate of pay
        let mut end_pay_rate = check_active_pay_rate(&payscale.rates_of_pay, end_date).clone();
    
        // Alter date_time to end date
        end_pay_rate.date_time = end_date.format("%Y-%m-%d").to_string();
        
        // Add our end_date to vec
        relevant_rates_of_pay.push(&end_pay_rate);

        // Create vec of PayPeriods
        let mut pay_periods: Vec<PayPeriod> = Vec::new();

        // loop through rates of pay and generate Vec<PayPeriod>
        let max_len = relevant_rates_of_pay.len() as usize;

        for (i, rp) in relevant_rates_of_pay.iter().enumerate() {

            // find the duration in hours within each rate_of_pay using max_len
            if i < (max_len - 1) {
                // Start at our start date
                let period_start = convert_string_to_naive_date(
                    &relevant_rates_of_pay[i].date_time); 
    
                // identify the end date
                let period_end = convert_string_to_naive_date(
                    &relevant_rates_of_pay[i + 1].date_time);

                // find duration in hours
                let duration = period_end.signed_duration_since(period_start);

                // take raw calendar days and get working hours (approximation)
                // days / 7.0 (weeks) * 5/0 (working days) * 7.5 (hours per workday)
                let days = (duration.num_days() as f64 / 7.0 * 5.0).round();

                let hours = days * 7.5;

                // determine rate of pay for period    
                let target_salary = rp.salary.get(step as usize -1);

                let target_salary = match target_salary {
                    Some(b) => *b as f64,
                    None => 0.0,
                };

                let pay_for_period = (target_salary / (260.0 * 7.5)) * hours as f64;

                let pay_for_period = round_to_2_decimal_points(pay_for_period);

                // create pay_period
                let p = PayPeriod {
                    start_date: period_start,
                    end_date: period_end,
                    duration_in_days: round_to_2_decimal_points(hours / 7.5),
                    duration_in_hours: round_to_2_decimal_points(hours),
                    annual_rate: target_salary,
                    hourly_rate: round_to_2_decimal_points(target_salary / (260.0 * 7.5)), 
                    salary: pay_for_period, 
                };
    
                // add pay_period to vec
                pay_periods.push(p);
            };


        };
        Some(pay_periods)
    }
    /// Directly returns the today's in force salary for a level and step within the group
    /// without needing to access pay scales and rates of pay.
    /// Accepts level and step as integers as arguments.
    pub fn pay_in_force_for_level_and_step(&self, level: i32, step: i32) -> Option<&i32> {
        
        let payscale = self.pay_scales.iter().find(|p| p.level == level);

        let payscale = match payscale {
            Some(p) => p,
            None => return None
        };
        
        // get current date and structure for PartialOrd
        let today: DateTime<Local> = Local::now();
        let today: NaiveDate = NaiveDate::from_ymd(
            today.year(), 
            today.month(), 
            today.day());

        let target_rate = check_active_pay_rate(&payscale.rates_of_pay, today);

        let current_salary = target_rate.salary.get(step as usize -1);
        
        current_salary
    }
    /// Directly returns the pay at a specified date for a level and step within the group
    /// without needing to access pay scales and rates of pay.
    /// Accepts level and step as integers and date in a YYYY-MM-DD string as arguments.
    pub fn pay_on_date_for_level_and_step(&self, level: i32, step: i32, date: String) -> Option<&i32> {
        
        let payscale = self.pay_scales.iter().find(|p| p.level == level);

        let payscale = match payscale {
            Some(p) => p,
            None => return None
        };

        // Error here if level not applied for all groups
        
        // get target date and structure for PartialOrd
        let target_date: NaiveDate = NaiveDate::parse_from_str(
            date.as_str(),
            "%Y-%m-%d").unwrap();

        let target_rate = check_active_pay_rate(&payscale.rates_of_pay, target_date);

        let target_salary = target_rate.salary.get(step as usize -1);
        
        target_salary
    }
}