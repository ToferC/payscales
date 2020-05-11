use juniper::{RootNode, FieldResult};

use crate::models::{Group, NewGroup, PayScale, PayRow};

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn groups() -> Vec<Group> {
        vec![
            Group {
                identifier: "ec".to_owned(),
                name: "Economics and Social".to_owned(),
                url: "https://tbs-sct.gc.ca".to_owned(),
                payScales: Vec::new(),
            },
            Group {
                identifier: "cs".to_owned(),
                name: "Computer Science".to_owned(),
                url: "https://tbs-sct.gc.ca".to_owned(),
                payScales: Vec::new(),
            }
        ]
    }
    fn group(identifier: String) -> FieldResult<Group> {
        Ok(Group {
            identifier: "cs".to_owned(),
            name: "Computer Science".to_owned(),
            url: "https://tbs-sct.gc.ca".to_owned(),
            payScales: Vec::new(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {
    fn createGroup(new_group: NewGroup) -> FieldResult<Group> {
        Ok(Group {
            identifier: String::from("fb"),
            name: String::from("Border Services"),
            url: String::from("https://tbs-sct.gc.ca/fb"),
            payScales: Vec::new(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {} )
}

