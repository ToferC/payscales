use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;
use crate::utilities::check_active_pay_rate;

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

        let target_rate = check_active_pay_rate(&self.rates_of_pay, today);

        &target_rate

    }
    
    // Accepts a YY-MM-DD string and returns the payrate_of_pay in effect for the date provided, past, present or future.
    pub fn rate_of_pays_for_date(&self, date: String) -> &RateOfPay {

        // get target date and structure for PartialOrd
        let target_date: NaiveDate = NaiveDate::parse_from_str(
            date.as_str(),
            "%Y-%m-%d").unwrap();
        
        let target_rate = check_active_pay_rate(&self.rates_of_pay, target_date);
        
        &target_rate
    }
}