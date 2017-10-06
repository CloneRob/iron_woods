use std::collections::HashMap;
use ::feature::{Value, Label};

trait Splitable {
    fn split_at_index(&self, index: usize) -> (Subset, Subset);
    fn split_at_threshold<T>(&self, threshold: T) -> (Subset, Subset);
}
pub struct Dataset {
    pub mapping: HashMap<usize, String>,
    pub set: Vec<Vec<Value>>,
    pub labels: Vec<Label>,
}

impl Splitable for Dataset {
    fn split_at_index(&self, index: usize) -> (Subset, Subset) {
        let mut left_subset: Vec<&[Value]> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<&[Value]> = Vec::with_capacity(self.set.len());

        for feature in &self.set {
            let (left_slice, right_slice) = {
                (&feature[..index], &feature[index..])
            };
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }
        let (left_labels, right_labels) = {
            (&self.labels[..index], &self.labels[index..])
        };
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }

    fn split_at_threshold<Value>(&self, threshold: Value) -> (Subset, Subset) {
        unimplemented!()
    }
}

pub struct Subset<'s> {
    pub set: Vec<&'s [Value]>,
    pub labels: &'s [Label],
}

impl<'s> Subset<'s> {
    fn new(set: Vec<&'s [Value]>, labels: &'s [Label]) -> Subset<'s> {
        Subset {
            set: set,
            labels: labels
        }
    }
}


impl<'s> Splitable for Subset<'s> {
    fn split_at_index(&self, index: usize) -> (Subset, Subset) {
        let mut left_subset: Vec<&[Value]> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<&[Value]> = Vec::with_capacity(self.set.len());

        for feature in &self.set {
            let (left_slice, right_slice) = {
                (&feature[..index], &feature[index..])
            };
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }
        let (left_labels, right_labels) = {
            (&self.labels[..index], &self.labels[index..])
        };
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }

    fn split_at_threshold<Value>(&self, threshold: Value) -> (Subset, Subset) {
        unimplemented!()
    }
}