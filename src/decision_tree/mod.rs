use ::feature::Label;
use ::data::Dataset;

pub struct DecisionTree {
    root: Option<DecisionNode>,
    min_split: usize,
    min_impurity: f32,
    max_depth: usize,
    cost_fn: fn(&[f32]) -> f32,
    leaf_fn: fn() -> f32,
}


impl DecisionTree {
    fn build_tree(&mut self, dataset: &Dataset) {
        for feature in &dataset.set {
            for (index, value) in feature.into_iter().enumerate() {
                // let left, right =     
            }
        }
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