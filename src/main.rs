use std::sync::Arc;
use std::collections::HashMap;
struct Class {}
enum Node {
    Intermediate{ threshold: f32},
    Leaf(Class)
}
struct CART {
    root: Box<Node>,
}
struct Dataset(HashMap<String, Arc<Feature>>);
trait Feature{}
struct Entropy {}

fn main() {
    println!("Hello, world!");
}
