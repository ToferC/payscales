use serde::{Deserialize};
use chrono::prelude::*;

use crate::DataBase;
use crate::utilities::{convert_string_to_naive_date, round_to_2_decimal_points};
use super::enums::{Period};

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
    pub fn salary(&self, step: i32, period: Period) -> Option<f64> {
        let base = self.salary.get(step as usize -1);

        let base = match base {
            Some(b) => *b as f64,
            None => 0.0,
        };

        let period_salary = match period {
            Period::Annual => base as f64,
            Period::Weekly => base as f64 / 26.0,
            Period::Daily => base as f64 / 260.0,
            Period::Hourly => base as f64 / (260.0 * 7.5)
        };

        if period_salary == 0.0 {
            None
        } else {
            Some(round_to_2_decimal_points(period_salary))
        }
    }
}