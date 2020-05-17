use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;

use super::rate_of_pay::RateOfPay;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct PayScale {
    pub name: String,
    pub level: i32,
    pub steps: i32,
    pub rates_of_pay: Vec<RateOfPay>,
}

#[juniper::object(Context = DataBase)]
/// A payscale containing a specific level and agreed pay rates for a period of time and pay steps.
/// This would contain all data for an EC-04, for example, including the changes to pay according to the collective agreeement
/// and the annual pay steps within the specific agreement.
/// Of note, many payScales are behind the current date and/or are being negotiated at any point in time. 
impl PayScale {
    /// The name of the payscale, e.g.: EC-04
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// The level of the payscale as an integer, e.g.: 4
    pub fn level(&self) -> i32 {
        self.level
    }
    /// The number of steps in a payscale - e.g., there are 5 steps in the EC-04 payscale
    pub fn steps(&self) -> i32 {
        self.steps
    }
        /// The rates of pay in a specific payscale. 
        /// E.g., the EC-04 payscale has seven adjustments between June 22, 2017 and June 22, 2021. 
        /// Each rate of pay consists of a date that it comes into force, a number of steps and a salary for each step.
    pub fn rates_of_pay(&self) -> &Vec<RateOfPay> {
        &self.rates_of_pay
    }
    
    /// Returns the rate of pay for today's date. 
    /// Checks today's date against the effective date of a rate of pay and potential future rates of pay. 
    /// This returns the rates of pay that are currently active for the payscale.
    pub fn current_rates_of_pay(&self) -> &RateOfPay {

        // get current date and structure for PartialOrd
        let today: DateTime<Local> = Local::now();
        let today: NaiveDate = NaiveDate::from_ymd(
            today.year(), 
            today.month(), 
            today.day());

        let mut target = 0;
        let end_index = self.rates_of_pay.len() - 1;
        
        for (i,rate_of_pay) in self.rates_of_pay.iter().enumerate() {

            if i < end_index {
                // set start_date forrate_of_pay 
                let start_date = NaiveDate::parse_from_str(
                    &self.rates_of_pay[i].date_time,
                    "%Y-%m-%d").unwrap();
    
                // get the end date forrate_of_pay
                let end_date = NaiveDate::parse_from_str(
                    &self.rates_of_pay[i+1].date_time.as_str(),
                    "%Y-%m-%d").unwrap();

                // Check to see if today's date is withing therate_of_pay start and end dates

                if today > start_date && today <= end_date {
                    // set target to current index
                    target = i;
                    break
                }
            
            } else {
                // Current date isn't within an in forcerate_of_pay
                // So we should use the lastrate_of_pay available
                target = end_index;
            }
            
        }
        
        // returnrate_of_pay for this date
        &self.rates_of_pay[target]
    }

    // Accepts a YY-MM-DD string and returns the payrate_of_pay in effect for the date provided, past, present or future.
    pub fn rate_of_pays_for_date(&self, date: String) -> &RateOfPay {

        // get current date and structure for PartialOrd
        let target_date: NaiveDate = NaiveDate::parse_from_str(
            date.as_str(),
            "%Y-%m-%d").unwrap();

        let mut target = 0;
        let end_index = self.rates_of_pay.len() - 1;
        
        for (i,rate_of_pay) in self.rates_of_pay.iter().enumerate() {

            if i < end_index {
                // set start_date forrate_of_pay 
                let start_date = NaiveDate::parse_from_str(
                    &self.rates_of_pay[i].date_time,
                    "%Y-%m-%d").unwrap();
    
                // get the end date forrate_of_pay
                let end_date = NaiveDate::parse_from_str(
                    &self.rates_of_pay[i+1].date_time.as_str(),
                    "%Y-%m-%d").unwrap();

                // Check to see if today's date is withing therate_of_pay start and end dates

                if target_date > start_date && target_date <= end_date {
                    // set target to current index
                    target = i;
                    break
                }
            
            } else {
                // Current date isn't within an in forcerate_of_pay
                // So we should use the lastrate_of_pay available
                target = end_index;
            }
        }
        
        // returnrate_of_pay for this date
        &self.rates_of_pay[target]
    }
}