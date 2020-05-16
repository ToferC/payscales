use serde::{Deserialize};
use crate::DataBase;

use super::payscale::PayScale;
use super::increment::Increment;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all= "snake_case")]
pub struct Group {
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
/// A pay group as defined by a collective agreement
impl Group {
    /// The Group's Name
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    /// The two-letter identifier for the group
    pub fn identifier(&self) -> &str {
        self.identifier.as_str()
    }
    /// The URL for the collective agreement
    pub fn url(&self) -> &str {
        self.url.as_str()
    }
    /// Vector of payscales for the group
    pub fn payscales(&self) -> &Vec<PayScale> {
        &self.pay_scales
    }
    // Date the collective agreement was scraped
    pub fn date_scraped(&self) -> &str {
        &self.date_scraped.as_str()
    }

    /// Returns a payscale for a specific level within the group
    pub fn payscale_for_level(&self, level: i32) -> Option<&PayScale> {
        self.pay_scales.iter().find(|p| p.level == level)
    }
}