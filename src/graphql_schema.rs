use juniper::{RootNode, FieldResult, EmptyMutation};

use crate::models::{Group};

use crate::utilities::read_file_to_group;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn groups() -> Vec<Group> {
        vec![
            Group {
                identifier: "ec".to_owned(),
                name: "Economics and Social".to_owned(),
                url: "https://tbs-sct.gc.ca/ec".to_owned(),
                pay_scales: Vec::new(),
                date_scraped: "2020-05-01".to_owned()
            },
            Group {
                identifier: "cs".to_owned(),
                name: "Computer Science".to_owned(),
                url: "https://tbs-sct.gc.ca/cs".to_owned(),
                pay_scales: Vec::new(),
                date_scraped: "2020-05-01".to_owned()
            }
        ]
    }
    
    fn group(identifier: String) -> FieldResult<Group> {

        let g = read_file_to_group(format!("./data/{}.json", identifier).as_str()).unwrap();
        
        Ok(g)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new())
}

