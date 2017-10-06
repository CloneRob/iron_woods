use ::feature::Label;
use ::data::Dataset;

pub struct DecisionTree {
    root: Option<DecisionNode>,
    min_split: usize,
    min_impurity: f32,
    max_depth: usize,
    // impurity_fn: fn(&[Feature<Box<V>>], &[Feature<Box<V>>]) -> f32,
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

fn entropy() -> f32 {
    unimplemented!()
}

fn gini_index() -> f32 {
    unimplemented!()
}