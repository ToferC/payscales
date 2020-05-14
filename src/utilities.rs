use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use std::io::prelude::*;
use std::path::Path;
use serde_json::{Value, from_reader};

use crate::models::Group;

pub fn read_file_to_group(filepath: &str) -> Result<Group, Box<dyn Error>> {
    let file = File::open(filepath).expect("could not open file");
    let reader = BufReader::new(file);
    
    let g = serde_json::from_reader(reader)?;

    Ok(g)
}

pub fn load_json_files() -> Result<Box<Vec<Group>>, Box<dyn Error>> {
    let mut groups: Vec<Group> = Vec::new();

    let paths = fs::read_dir("./data").unwrap();
    
    for path in paths {
        let file_name = path.unwrap().file_name();
        let file_string = file_name.to_str().unwrap();
        println!("{:?}", file_string);
        let g = read_file_to_group(format!("./data/{}",file_string).as_str())?;
        groups.push(g)
    }

    Ok(Box::new(groups))
}