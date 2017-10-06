use std::cmp;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::{HashSet, HashMap};

#[derive(PartialEq, PartialOrd, Clone, Debug, Hash)]
pub struct Label(u32);
impl Eq for Label {}
impl Ord for Label {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Label(lhs) = self;
        let &Label(rhs) = other;
        lhs.cmp(&rhs)
    }
}

fn label_distribution(label_slice: &[Label]) -> Vec<f32> {
    let norm_const = label_slice.len() as f32;
    let mut counting_map: HashMap<&Label, u32> = HashMap::new();
    for l in label_slice {
        *counting_map.entry(l).or_insert(0) += 1;
    }
    counting_map.values().map(|v| *v as f32 / norm_const).collect()
}

#[derive(PartialEq, PartialOrd, Clone, Debug, Hash)]
pub struct Value(i32);

impl Eq for Value {}
impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let &Value(lhs) = self;
        let &Value(rhs) = other;
        lhs.cmp(&rhs)
    }
}

pub struct Feature {
    values: Vec<Value>,
}

impl Feature {}
