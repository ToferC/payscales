use serde::{Deserialize};
use chrono::prelude::*;

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

    
    pub fn increments(&self) -> &Vec<Increment> {
        &self.increments
    }
    
    pub fn current_pay(&self) -> &Increment {

        // get current date and structure for PartialOrd
        let today: DateTime<Local> = Local::now();
        let today: NaiveDate = NaiveDate::from_ymd(
            today.year(), 
            today.month(), 
            today.day());

        let target = 0;
        let end_index = self.increments.len() - 1;
        
        for (i, increment) in self.increments.iter().enumerate() {

            if i < end_index {
                // set start_date for increment 
                let start_date = NaiveDate::parse_from_str(
                    &self.increments[i].date_time,
                    "%Y-%m-%d").unwrap();
    
                // get the end date for increment
                let end_date = NaiveDate::parse_from_str(
                    &self.increments[i+1].date_time.as_str(),
                    "%Y-%m-%d").unwrap();

                // Check to see if today's date is withing the increment start and end dates

                if today > start_date && today <= end_date {
                    // set target to current index
                    let target = i;
                    break
                }
            
            } else {
                // Current date isn't within an in force increment
                // So we should use the last increment available
                let target = end_index;
            }
            
        }
        
        // return increment for this date
        &self.increments[target]
    }

    // Accepts a YY-MM-DD string and returns the pay increment in effect for that date
    pub fn increments_for_date(&self, date: String) -> &Increment {

        // get current date and structure for PartialOrd
        let target_date: NaiveDate = NaiveDate::parse_from_str(
            date.as_str(),
            "%Y-%m-%d").unwrap();

        let target = 0;
        let end_index = self.increments.len() - 1;
        
        for (i, increment) in self.increments.iter().enumerate() {

            if i < end_index {
                // set start_date for increment 
                let start_date = NaiveDate::parse_from_str(
                    &self.increments[i].date_time,
                    "%Y-%m-%d").unwrap();
    
                // get the end date for increment
                let end_date = NaiveDate::parse_from_str(
                    &self.increments[i+1].date_time.as_str(),
                    "%Y-%m-%d").unwrap();

                // Check to see if today's date is withing the increment start and end dates

                if target_date > start_date && target_date <= end_date {
                    // set target to current index
                    let target = i;
                    break
                }
            
            } else {
                // Current date isn't within an in force increment
                // So we should use the last increment available
                let target = end_index;
            }
            
        }
        
        // return increment for this date
        &self.increments[target]
    }
}