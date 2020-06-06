use serde::{Deserialize};
use chrono::prelude::*;

use crate::models::GroupID;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct PayPeriod {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub work_days: f64,
    pub work_hours: f64,
    pub step: i32,
    pub hourly_rate: f64,
    pub annual_rate: f64,
    pub salary: f64,
    pub identifier: GroupID,
    pub level: i32,
}

#[juniper::object()]
/// IMPORTANT: IN TESTING
/// A Pay Period represents a series of time periods and the approximate gross pay expect in each period
/// It is based on the pay rate in force for the dates in question at a current level and step.
/// If you want to track different steps, you will need to run multiple instances of PayAtLevelAndStepBetweenDates
/// in your query.
impl PayPeriod {
    /// The start date for a pay period in YYYY-MM-DD.
    pub fn start_date(&self) -> &NaiveDate {
        &self.start_date
    }

    /// The end date for a pay period in YYYY-MM-DD.
    pub fn end_date(&self) -> &NaiveDate {
        &self.end_date
    }
    /// The duration in hours for a pay period.
    pub fn work_hours(&self) -> &f64 {
        &self.work_hours
    }
    /// The duration in days for a pay period.
    pub fn work_days(&self) -> &f64 {
        &self.work_days
    }

    /// The active pay step for a pay period.
    pub fn step(&self) -> &i32 {
        &self.step
    }

    /// The active pay level for a pay period.
    pub fn level(&self) -> &i32 {
        &self.level
    }

    /// The identifier for a pay period.
    pub fn identifier(&self) -> &GroupID {
        &self.identifier
    }

    /// The hourly pay rate for a pay period.
    pub fn hourly_rate(&self) -> &f64 {
        &self.hourly_rate
    }

    /// The annual pay rate for a pay period.
    pub fn annual_rate(&self) -> &f64 {
        &self.annual_rate
    }

    /// The gross salary (approximate) for a pay period
    pub fn salary(&self) -> Option<f64> {
        
        if self.salary == 0.0 {
            None
        } else {
            Some(self.salary)
        }
    }
}