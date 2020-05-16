use serde::{Deserialize};
use crate::DataBase;

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
        self.salary[step as usize -1]
    }
}