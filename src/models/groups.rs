use juniper::{GraphQLObject};
use serde::{Deserialize};

#[derive(GraphQLObject, Deserialize)]
#[serde(rename_all= "snake_case")]
#[graphql(description = "A pay group")]
pub struct Group {
    /// The Group's Name
    pub name: String,
    /// The two-letter identifier for the group
    pub identifier: String,
    /// The URL for the collective agreement
    pub url: String,
    /// Vector of payscales for the group
    pub pay_scales: Vec<PayScale>,
}

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
}

#[derive(GraphQLObject, Deserialize)]
#[serde(rename_all= "snake_case")]
#[graphql(description = "Salary tables for a pay group and level")]
pub struct PayScale {
    pub name: String,
    pub steps: i32,
    pub pay_rows: Vec<PayRow>,
}

impl PayScale {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn steps(&self) -> i32 {
        self.steps
    }

    pub fn pay_rows(&self) -> &Vec<PayRow> {
        &self.pay_rows
    }

    pub fn pay_row_for_date(&self, date: String) -> Option<&PayRow> {
        self.pay_rows.iter().find(|p| p.date_time == date)
    }
}


#[derive(GraphQLObject, Deserialize)]
#[serde(rename_all= "snake_case")]
#[graphql(description = "Salary row for a date step")]
pub struct PayRow {
    pub date_time: String,
    pub salary: Vec<i32>,
}

impl PayRow {
    pub fn date_time(&self) -> &str {
        self.date_time.as_str()
    }

    pub fn salary(&self, step: usize) -> i32 {
        self.salary[step+1]
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