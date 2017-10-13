use std::cmp;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Index, RangeTo, RangeFrom};
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

pub fn label_distribution(label_slice: &[&Label]) -> Vec<f32> {
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

// pub struct Feature<T> {
//     data: FeatureType,
//     unique_values: HashSet<T>,
// }
// pub enum FeatureType {
//     Discrete(Vec<i32>),
//     Continuous(Vec<f32>),
// }


// impl Feature {
//     fn set_uniques(&mut self) {
//         if let Some(ref uv) = self.unique_values {
//             return
//         } else {
//             let uv: HashSet<Value> = self.values.iter().map(|v| v.clone()).collect();
//             self.unique_values = Some(uv);
//         }
//     }
// }

// pub struct FeatureSlice<'s> {
//     values: &'s [Value],
//     unique_values: Option<HashSet<Value>>,
// }
