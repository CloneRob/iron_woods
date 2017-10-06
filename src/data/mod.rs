use std::collections::HashMap;
use ::feature::{Value, Label};

pub struct Dataset {
    pub mapping: HashMap<usize, String>,
    pub set: Vec<Vec<Value>>,
    pub labels: Vec<Label>,
}