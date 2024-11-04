use crate::value::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub value: Option<Value>,
    pub children: HashMap<char, Arc<Node>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            value: None,
            children: HashMap::new(),
        }
    }
}
