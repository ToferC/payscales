use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct ActiveRateOfPay {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub step: i32,
    pub salary: i32,
}

#[juniper::object(Context = DataBase)]
/// A Rate of Pay for a collective agreement at a point in time across several pay steps.
/// Includes a date_time for when the rate of pay comes into force and an array of salary steps.
impl ActiveRateOfPay {
    /// The date_time at which a rate of pay comes into force.
    pub fn start_date(&self) -> NaiveDate {
        self.start_date
    }

    pub fn end_date(&self) -> NaiveDate {
        self.end_date
    }

    pub fn step(&self) -> &i32 {
        &self.step
    }

    /// The range of salary steps within a rate of pay. An array of integers.
    pub fn salary(&self) -> &i32 {
        &self.salary
    }
}