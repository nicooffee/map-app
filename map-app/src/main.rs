#[allow(dead_code)]
extern crate termion;
mod engine;
use engine::engine::{run,new};


fn main() {
    let (s,m) = new();
    run(s,m);
}
