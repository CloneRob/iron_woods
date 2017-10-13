use std::collections::HashMap;

use ::feature::{Value, Label};


pub trait Splitable {
    fn split_at_index(&self, index: usize) -> (Subset, Subset);
    fn split_at_threshold(&self, target_feature: usize, threshold: Value) -> (Subset, Subset);
}

pub struct Dataset {
    pub mapping: HashMap<usize, String>,
    pub set: Vec<Vec<Value>>,
    pub labels: Vec<Label>,
}

impl Splitable for Dataset {
    fn split_at_index(&self, index: usize) -> (Subset, Subset) {
        let mut left_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());

        for feature in &self.set {
            let (mut left_slice, mut right_slice) = (Vec::new(), Vec::new());
            for (i, v) in feature.into_iter().enumerate() {
                if i <= index {
                    left_slice.push(v)
                } else {
                    right_slice.push(v)
                }
            }
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }
        let mut left_labels = Vec::new();
        let mut right_labels = Vec::new();

        for (i, l) in self.labels.iter().enumerate() {
            if i <= index {
                left_labels.push(l)
            } else {
                right_labels.push(l)
            }
        }
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }

    fn split_at_threshold(&self, target_feature: usize, threshold: Value) -> (Subset, Subset) {
        let mut left_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut left_index_set = Vec::new();
        let mut right_index_set = Vec::new();

        for (i, value) in self.set[target_feature].iter().enumerate() {
            if value <= &threshold {
                left_index_set.push(i);
            } else {
                right_index_set.push(i);
            }
        }

        for feature in &self.set {
            let mut left_slice = Vec::new();
            for li in left_index_set.iter() {
                left_slice.push(&feature[*li])
            }
            let mut right_slice = Vec::new();
            for ri in right_index_set.iter() {
                right_slice.push(&feature[*ri])
            }
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }

        let mut left_labels = Vec::new();
        for li in left_index_set {
            left_labels.push(&self.labels[li])
        }
        let mut right_labels = Vec::new();
        for ri in right_index_set {
            right_labels.push(&self.labels[ri])
        }
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }
}

pub struct Subset<'s> {
    pub set: Vec<Vec<&'s Value>>,
    pub labels: Vec<&'s Label>,
}

impl<'s> Subset<'s> {
    fn new(set: Vec<Vec<&'s Value>>, labels: Vec<&'s Label>) -> Subset<'s> {
        Subset {
            set: set,
            labels: labels
        }
    }
}


impl<'s> Splitable for Subset<'s> {
    fn split_at_index(&self, index: usize) -> (Subset, Subset) {
        let mut left_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());

        for feature in &self.set {
            let (mut left_slice, mut right_slice) = (Vec::new(), Vec::new());
            for (i, v) in feature.into_iter().enumerate() {
                if i <= index {
                    left_slice.push(*v)
                } else {
                    right_slice.push(*v)
                }
            }
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }
        let mut left_labels = Vec::new();
        let mut right_labels = Vec::new();

        for (i, l) in self.labels.iter().enumerate() {
            if i <= index {
                left_labels.push(*l)
            } else {
                right_labels.push(*l)
            }
        }
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }


    fn split_at_threshold(&self, target_feature: usize, threshold: Value) -> (Subset, Subset) {
        let mut left_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut right_subset: Vec<Vec<&Value>> = Vec::with_capacity(self.set.len());
        let mut left_index_set = Vec::new();
        let mut right_index_set = Vec::new();

        for (i, value) in self.set[target_feature].iter().enumerate() {
            if *value <= &threshold {
                left_index_set.push(i);
            } else {
                right_index_set.push(i);
            }
        }

        for feature in &self.set {
            let mut left_slice = Vec::new();
            for li in &left_index_set {
                left_slice.push(feature[*li])
            }
            let mut right_slice = Vec::new();
            for ri in &right_index_set {
                right_slice.push(feature[*ri])
            }
            left_subset.push(left_slice);
            right_subset.push(right_slice);
        }

        let mut left_labels = Vec::new();
        for li in &left_index_set {
            left_labels.push(self.labels[*li])
        }
        let mut right_labels = Vec::new();
        for ri in right_index_set {
            right_labels.push(self.labels[ri])
        }
        (Subset::new(left_subset, left_labels), Subset::new(right_subset, right_labels))
    }
}