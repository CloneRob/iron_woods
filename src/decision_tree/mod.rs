use ::feature;
use ::feature::Label;
use ::data::{Dataset, Splitable};
use std::default::Default;

pub struct DecisionTree {
    root: Option<DecisionNode>,
    min_split: usize,
    min_impurity: f32,
    max_depth: usize,
    cost_fn: fn(&[f32]) -> f32,
    leaf_fn: fn() -> f32,
}

impl Default for DecisionTree {
    fn default() -> DecisionTree {
        DecisionTree {
            root: None,
            min_split: 0,
            min_impurity: 0.5,
            max_depth: 15,
            cost_fn: entropy,
            leaf_fn: gini_index,
        }
    }
}


impl DecisionTree {
    fn build_tree(&mut self, dataset: &Dataset, depth: usize) -> DecisionTree {
        for (feature_index, feature) in dataset.set.iter().enumerate() {
            for (value_index, value) in feature.iter().enumerate() {
                let (left, right) = dataset.split_at_threshold(feature_index, value.clone());
                let left_distribution = feature::label_distribution(&left.labels[..]);
                let left_entropy = (self.cost_fn)(&left_distribution[..]);

                let right_distribution = feature::label_distribution(&right.labels[..]);
                let right_entropy = (self.cost_fn)(&right_distribution[..]);
            }
        }
        DecisionTree::default()
    }
}

pub enum DecisionNode {
    Leaf (Label),
    Intermediate {
        threshold: f32,
        feature_index: usize,
        left: Box<DecisionNode>,
        right: Box<DecisionNode>,
    }
}

fn entropy(label_dist: &[f32]) -> f32 {
    let mut entropy = 0.0;
    for p in label_dist {
        entropy += p * p.ln();
    }
    entropy
}


fn gini_index() -> f32 {
    unimplemented!()
}