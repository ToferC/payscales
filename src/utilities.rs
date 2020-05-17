use std::fs::File;
use std::error::Error;
use std::io::{BufReader};

use crate::models::{Group};

pub fn load_group_data() -> Result<Vec<Group>, Box<dyn Error>> {
    let file = File::open("./data/groups_data.json").expect("could not open file");
    let reader = BufReader::new(file);
    
    let group_data: Vec<Group> = serde_json::from_reader(reader).unwrap();

    Ok(group_data)
}