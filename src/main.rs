use std::sync::Arc;
use std::collections::HashMap;

type Class = u16;
trait Feature{}
struct Categorical {}
impl Feature for Categorical {}

struct Continuous {}
impl Feature for Continuous {}

struct Discrete {}
impl Feature for Discrete {}

// struct Class {}
struct Node {
    left: Box<Node>,
    right: Box<Node>,
    ntype: NodeType,
}
enum NodeType {
    Intermediate{ threshold: f32},
    Leaf(Class)
}
struct CART {
    root: Box<Node>,
}
struct Dataset(HashMap<String, Arc<Feature>>);
struct Entropy {}

fn main() {
    println!("Hello, world!");
}
