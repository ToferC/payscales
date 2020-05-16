use juniper::{RootNode, FieldResult, EmptyMutation};

use crate::models::{Group, GroupID};

use crate::DataBase;

pub struct QueryRoot;

#[juniper::object(Context = DataBase)]
impl QueryRoot {
    pub fn groups(
        context: &DataBase
    ) -> Vec<Group> {
        
        let groups = &context.groups.clone();

        groups.to_vec()
    }
    
    fn group(
        context: &DataBase,
        identifier: GroupID) -> FieldResult<&Group> {

        let g = &context.groups.iter().find(|g| g.identifier == identifier).unwrap();
        
        Ok(g)
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<DataBase>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, EmptyMutation::new())
}

