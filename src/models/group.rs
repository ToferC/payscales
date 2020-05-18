use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;
use crate::utilities::{convert_string_to_naive_date, check_active_pay_rate};

use super::payscale::PayScale;
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
    /// Date the collective agreement was scraped.
    pub fn date_scraped(&self) -> NaiveDate {
        convert_string_to_naive_date(&self.date_scraped)
    }
    /// Returns a payscale for a specific level within the group.
    pub fn payscale_for_level(&self, level: i32) -> Option<&PayScale> {
        self.pay_scales.iter().find(|p| p.level == level)
    }
    /// Directly returns the today's in force salary for a level and step within the group
    /// without needing to access pay scales and rates of pay.
    /// Accepts level and step as integers as arguments.
    pub fn pay_for_level_and_step_today(&self, level: i32, step: i32) -> i32 {
        
        let payscale = self.pay_scales.iter().find(|p| p.level == level).unwrap();
        
        // get current date and structure for PartialOrd
        let today: DateTime<Local> = Local::now();
        let today: NaiveDate = NaiveDate::from_ymd(
            today.year(), 
            today.month(), 
            today.day());

        let target_rate = check_active_pay_rate(&payscale.rates_of_pay, today);

        let current_salary = target_rate.salary[step as usize -1];
        
        current_salary
    }
    /// Directly returns the pay at a specified date for a level and step within the group
    /// without needing to access pay scales and rates of pay.
    /// Accepts level and step as integers and date in a YY-MM-DD string as arguments.
    pub fn pay_for_level_and_step_on_date(&self, level: i32, step: i32, date: String) -> i32 {
        
        let payscale = self.pay_scales.iter().find(|p| p.level == level).unwrap();
        
        // get target date and structure for PartialOrd
        let target_date: NaiveDate = NaiveDate::parse_from_str(
            date.as_str(),
            "%Y-%m-%d").unwrap();

        let target_rate = check_active_pay_rate(&payscale.rates_of_pay, target_date);

        let target_salary = target_rate.salary[step as usize -1];
        
        target_salary
    }
}