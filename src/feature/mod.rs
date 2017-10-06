use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct Label {}
#[derive(PartialEq, PartialOrd, Clone, Debug, Hash)]
pub struct Value(i32);

impl Eq for Value {}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[test]
fn sorting_test() {
    let float_vec: Vec<i32> = vec![0, 3, 3, 3, 7, 223, -13];
    let mut value_vec: Vec<Value> = float_vec.into_iter().map(|v| Value(v)).collect();
    value_vec.sort();

}

#[test]
fn hashset_test() {
    use std::collections::HashSet;
    let int_vec: Vec<i32> = vec![0, 3, 3, 3, 7, 223, -13];
    let value_vec: Vec<Value> = int_vec.into_iter().map(|v| Value(v)).collect();
    let hash_set: HashSet<Value> = value_vec.into_iter().collect();
}

#[test]
fn hashing_test() {
    use std::collections::HashMap;
    let mut value_hashmap = HashMap::new();
    let v1 = Value(13);
    let v2 = Value(15);

    value_hashmap.insert(v1, String::from("Value 1"));
    value_hashmap.insert(v2, String::from("Value 2"));
}

// #[derive(PartialEq, PartialOrd, Clone)]
// pub enum Feature {
//     Continuous(Vec<f32>),
//     Discrete(Vec<i32>),
//     Categorical(Vec<u32>),
// }

// impl Eq for Feature {}
// impl Ord for Feature {

// }
