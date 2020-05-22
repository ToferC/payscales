use serde::{Deserialize};
use chrono::prelude::*;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct PayPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub duration_in_days: f64,
    pub duration_in_hours: f64,
    pub hourly_rate: f64,
    pub salary: f64,
}

#[juniper::object()]
/// IMPORTANT: IN TESTING
/// A Pay Period represents a series of time periods and the approximate gross pay expect in each period
/// It is based on the pay rate in force for the dates in question at a current level and step.
/// If you want to track different steps, you will need to run multiple instances of PayAtLevelAndStepBetweenDates
/// in your query.
impl PayPeriod {
    /// The date_time at which a rate of pay comes into force.
    pub fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    pub fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }

    pub fn duration_in_hours(&self) -> &f64 {
        &self.duration_in_hours
    }

    pub fn duration_in_days(&self) -> &f64 {
        &self.duration_in_days
    }

    pub fn hourly_rate(&self) -> &f64 {
        &self.hourly_rate
    }

    /// The range of salary steps within a rate of pay. An array of integers.
    pub fn salary(&self) -> Option<f64> {
        
        if self.salary == 0.0 {
            None
        } else {
            Some(self.salary)
        }
    }
}