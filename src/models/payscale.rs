use serde::{Deserialize};
use crate::DataBase;

use super::increment::Increment;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct PayScale {
    pub name: String,
    pub level: i32,
    pub steps: i32,
    pub current_pay: Vec<i32>,
    pub increments: Vec<Increment>,
}

#[juniper::object(Context = DataBase)]
impl PayScale {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn steps(&self) -> i32 {
        self.steps
    }

    pub fn current_pay(&self) -> Box<&Vec<i32>> {
        Box::new(&self.current_pay)
    }

    pub fn increments(&self) -> &Vec<Increment> {
        &self.increments
    }

    pub fn increments_for_date(&self, date: String) -> Option<&Increment> {
        self.increments.iter().find(|i| i.date_time == date)
    }
}