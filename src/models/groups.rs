use serde::{Deserialize};
use std::collections::HashMap;
use crate::DataBase;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct Group {
    /// The Group's Name
    pub name: String,
    /// The two-letter identifier for the group
    pub identifier: String,
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
impl Group {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }

    pub fn url(&self) -> &str {
        self.url.as_str()
    }

    pub fn payScales(&self) -> &Vec<PayScale> {
        &self.pay_scales
    }

    pub fn date_scraped(&self) -> &str {
        &self.date_scraped.as_str()
    }
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct PayScale {
    pub name: String,
    pub steps: i32,
    pub current_pay: Vec<i32>,
    pub increments: Vec<Increment>,
}

#[juniper::object(Context = DataBase)]
impl PayScale {
    pub fn name(&self) -> &str {
        self.name.as_str()
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

    pub fn increment_for_date(&self, date: String) -> Option<&Increment> {
        self.increments.iter().find(|p| p.date_time == date)
    }

    pub fn scale_for_level(&self, level: i32) -> i32 {
        // Not implemented
        42
    }
}


#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct Increment {
    pub date_time: String,
    pub salary: Vec<i32>,
}

#[juniper::object(Context = DataBase)]
impl Increment {
    pub fn date_time(&self) -> &str {
        self.date_time.as_str()
    }

    pub fn salary(&self, step: i32) -> i32 {
        self.salary[step as usize +1]
    }
}

/*
#[derive(GraphQLInputObject)]
#[graphql(description = "A pay group")]
pub struct NewGroup {
    pub name: String,
    pub identifier: String,
    pub url: String,
    pub payScales: Vec<String>,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Salary tables for a pay group and level")]
pub struct NewPayScale {
    pub name: String,
    pub steps: i32,
    pub payScale: Vec<PayRow>,
}

#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "Salary row for a date step")]
pub struct NewPayRow {
    pub date_time: String,
    pub salary: Vec<i32>,
}
*/