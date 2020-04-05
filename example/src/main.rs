#[macro_use]
extern crate rsobserve_attrs;

#[observe]
pub fn abc() {
    println!("Called from macro");
}

fn main() {
    println!("Hello, world!");
    abc()
}
