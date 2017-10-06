use std::collections::HashMap;
use std::collections::HashSet;
pub mod decision_tree;
pub mod feature;
pub mod data;

fn main() {
    println!("Hello, world!");
    let feature = vec![1, 2, 1, 3, 1, 5, 6, 2, 24, 11, 231];
    let unique_feature: HashSet<_> = feature.into_iter().collect();
    for i in unique_feature {
        println!("{}", i);
    }
}
