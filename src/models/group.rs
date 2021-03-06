use std::cmp;

use serde::{Deserialize};
use chrono::prelude::*;
use chrono::Duration;
use bdays::HolidayCalendar;

use crate::DataBase;
use crate::utilities::{
    convert_string_to_naive_date,
    check_active_pay_rate,
    return_active_pay_for_period,
    round_to_2_decimal_points
};

use crate::models::{PayScale, RateOfPay, ActiveRateOfPay};
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

    /// Returns a vector of PayPeriods representing the expected pay for a range of work days inclusive of two YYYY-MM-DD dates.
    /// For example, start_date: "2020-05-01" and end_date: "2020-05-05" would return pay for 1 day of 7.5 hours.
    /// The function returns work days and holidays (for which public servants receive pay), but not weekends.
    /// Also requires a level and step in integers to compute the requested pay.
    pub fn pay_at_level_and_step_between_dates(&self, level: i32, step: i32, start_date: NaiveDate, end_date: NaiveDate) -> Option<Vec<PayPeriod>> {
        let payscale = self.pay_scales.iter().find(|p| p.level == level);

        // set businessdays calendar to weekends only
        let cal = bdays::calendars::WeekendsOnly;

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

                // Set calculation start date at period -1 to include day 1 of period
                let calculation_start = period_start - Duration::days(1);
                
                // identify the end date
                let mut period_end = convert_string_to_naive_date(
                    &relevant_rates_of_pay[i + 1].date_time);
                    
                let mut calculation_end: NaiveDate = period_end;
                
                if i == max_len - 2 {
                    // Set calculation start date at period +1 to include last day
                    calculation_end += Duration::days(1);
                } else {
                    period_end -= Duration::days(1);
                }

                // get work days in period
                let cal = bdays::HolidayCalendarCache::new(
                    bdays::calendars::WeekendsOnly,
                    // Using calculation start and end here while keeping the display dates
                    // the same as the user query
                    calculation_start,
                    // add two days to calendar dt_max to capture potential weekend
                    // and one day to calendar to capture our inclusive range
                    calculation_end + Duration::days(3),
                );

                // get business days, excluding only weekends
                // add 1 day to period_end to get inclusive range
                let days = cal.bdays(period_start, period_end + Duration::days(1));

                let hours = days as f64 * 7.5;

                // determine rate of pay for period    
                let target_salary = rp.salary.get(step as usize -1);

                let target_salary = match target_salary {
                    Some(b) => *b as f64,
                    None => 0.0,
                };

                let pay_for_period = (target_salary / (261.0 * 7.5)) * hours as f64;

                let pay_for_period = round_to_2_decimal_points(pay_for_period);

                // create pay_period
                let p = PayPeriod {
                    start_date: period_start,
                    end_date: period_end,
                    work_days: round_to_2_decimal_points(hours / 7.5),
                    work_hours: round_to_2_decimal_points(hours),
                    step: step,
                    annual_rate: target_salary,
                    hourly_rate: round_to_2_decimal_points(target_salary / (261.0 * 7.5)), 
                    salary: pay_for_period, 
                    level: level,
                    identifier: self.identifier,
                };
    
                // add pay_period to vec
                pay_periods.push(p);
            };


        };
        Some(pay_periods)
    }

    /// Returns a vector of PayPeriods representing the expected pay including annual step increments for a range of work days inclusive of two YYYY-MM-DD dates.
    /// For example, start_date: "2020-05-01" and end_date: "2020-05-05" would return pay for 1 day of 7.5 hours.
    /// The function returns work days and holidays (for which public servants receive pay), but not weekends.
    /// Also requires a level in integers and an anniversary date in YYYY-MM-DD.
    /// If anniversary_date is after the start_date, it will be changed to be the same as the start date.
    pub fn pay_at_level_by_anniversary_date_between_dates(&self, level: i32, start_date: NaiveDate, end_date: NaiveDate, mut anniversary_date: NaiveDate) -> Option<Vec<PayPeriod>> {
        let payscale = self.pay_scales.iter().find(|p| p.level == level);

        // use payscales to generate active_rates_of_pay for period

        // set businessdays calendar to weekends only
        let cal = bdays::calendars::WeekendsOnly;

        let payscale = match payscale {
            Some(p) => p,
            None => return None
        };

        // determine step based on anniversary date
        let step: i32 = 0;

        // Create vec of anniversary dates
        let mut steps: Vec<(NaiveDate, NaiveDate)> = Vec::new();

        // Ensure that anniversary date isn't after the start date
        if anniversary_date > start_date {
            anniversary_date = NaiveDate::from_ymd(
                start_date.year(),
                start_date.month(),
                start_date.day());
        }

        // add years as NaiveDate to Vec from anniversary date to max anniversary 
        // Note that we are adding more steps than required so we can keep a loop going
        // in return_active_pay_for_periods
        for y in anniversary_date.year()..end_date.year() + cmp::max(payscale.steps -1, 2) {
            let s = NaiveDate::from_ymd(y, anniversary_date.month(), anniversary_date.day());
            let e = NaiveDate::from_ymd(y, anniversary_date.month(), anniversary_date.day()) + Duration::days(365);
            steps.push((s, e));
        }

        println!("{:?}", steps);

        // Check for crossing Rates of pay based on dates and add each date to vec
        let mut relevant_rates_of_pay: Vec<&ActiveRateOfPay> = Vec::new();

        // Get vec of periods with annual pay, step, start and end dates
        // This needs to get all pay periods in range and split them for anniversary date
        // We should do this
        let active_rates = return_active_pay_for_period(&payscale, steps, start_date, end_date);

        // Create vec of PayPeriods
        let mut pay_periods: Vec<PayPeriod> = Vec::new();

        // loop through rates of pay and generate Vec<PayPeriod>
        let max_len = active_rates.len() as usize;

        for (i, rp) in active_rates.iter().enumerate() {

            // find the duration in hours within each rate_of_pay using max_len
            if i < max_len {

                // Start at our start date
                let period_start = rp.start_date;

                // Set calculation start date at period -1 to include day 1 of period
                let calculation_start = period_start - Duration::days(1);
                
                // identify the end date
                let mut period_end = rp.end_date;
                    
                let mut calculation_end: NaiveDate = period_end;
                
                if i == max_len - 1 {
                    // Set calculation start date at period +1 to include last day
                    calculation_end += Duration::days(1);
                } else {
                    period_end -= Duration::days(1);
                }

                // get work days in period
                let cal = bdays::HolidayCalendarCache::new(
                    bdays::calendars::WeekendsOnly,
                    // Using calculation start and end here while keeping the display dates
                    // the same as the user query
                    calculation_start,
                    // add two days to calendar dt_max to capture potential weekend
                    // and one day to calendar to capture our inclusive range
                    calculation_end + Duration::days(3),
                );

                // get business days, excluding only weekends
                // add 1 day to period_end to get inclusive range
                let days = cal.bdays(period_start, period_end + Duration::days(1));

                let hours = days as f64 * 7.5;

                let pay_for_period = (rp.salary as f64 / (261.0 * 7.5)) * hours as f64;

                let pay_for_period = round_to_2_decimal_points(pay_for_period);

                // create pay_period
                let p = PayPeriod {
                    start_date: period_start,
                    end_date: period_end,
                    work_days: round_to_2_decimal_points(hours / 7.5),
                    work_hours: round_to_2_decimal_points(hours),
                    step: rp.step,
                    annual_rate: rp.salary as f64,
                    hourly_rate: round_to_2_decimal_points(rp.salary as f64 / (261.0 * 7.5)), 
                    salary: pay_for_period,
                    level: level,
                    identifier: self.identifier,
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