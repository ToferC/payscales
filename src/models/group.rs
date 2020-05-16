use serde::{Deserialize};
use crate::DataBase;

use super::payscale::PayScale;
use super::increment::Increment;

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

    pub fn payscales(&self) -> &Vec<PayScale> {
        &self.pay_scales
    }

    pub fn date_scraped(&self) -> &str {
        &self.date_scraped.as_str()
    }

    pub fn payscale_for_level(&self, level: i32) -> Option<&PayScale> {
        self.pay_scales.iter().find(|p| p.level == level)
    }
}