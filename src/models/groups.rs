use chrono::NaiveDate;
use std::collections::HashMap;
use juniper::{GraphQLObject, GraphQLInputObject};

#[derive(GraphQLObject)]
#[graphql(description = "A pay group")]
pub struct Group {
    /// The Group's Name
    pub name: String,
    /// The two-letter identifier for the group
    pub identifier: String,
    /// The URL for the collective agreement
    pub url: String,
    /// Vector of payscales for the group
    pub payScales: Vec<String>,
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
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A pay group")]
pub struct NewGroup {
    pub name: String,
    pub identifier: String,
    pub url: String,
    pub payScales: Vec<String>,
}

#[derive(GraphQLObject)]
#[graphql(description = "Salary tables for a pay group and level")]
pub struct PayScale {
    pub name: String,
    pub steps: i32,
    pub payScale: Vec<PayRow>,
}


#[derive(GraphQLObject)]
#[graphql(description = "Salary row for a date step")]
pub struct PayRow {
    pub date_time: String,
    pub salary: Vec<i32>,
}

/*
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