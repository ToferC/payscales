use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;
use crate::utilities::convert_string_to_naive_date;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct RateOfPay {
    pub date_time: String,
    pub salary: Vec<i32>,
}

#[juniper::object(Context = DataBase)]
/// A Rate of Pay for a collective agreement at a point in time across several pay steps.
/// Includes a date_time for when the rate of pay comes into force and an array of salary steps.
impl RateOfPay {
    /// The date_time at which a rate of pay comes into force.
    pub fn in_force(&self) -> NaiveDate {
        convert_string_to_naive_date(&self.date_time)
    }

    /// The range of salary steps within a rate of pay. An array of integers.
    pub fn salary(&self, step: i32) -> Option<&i32> {
        self.salary.get(step as usize -1)
    }
}