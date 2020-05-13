use std::fs;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader};
use std::io::prelude::*;
use std::path::Path;
use serde_json::{Value, from_reader};

use crate::models::Group;

pub fn read_file_to_group(filepath: &str) -> Result<Group, Box<Error>> {
    let file = File::open(filepath).expect("could not open file");
    let mut reader = BufReader::new(file);
    
    let g = serde_json::from_reader(reader)?;

    Ok(g)
}