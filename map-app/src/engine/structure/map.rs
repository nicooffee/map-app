mod node;
use node::Node;
use super::scenario::Scenario;
use std::collections::HashMap;
use termion::{
    color
};
const ORIGIN: (i32,i32) = (0,0);

pub struct Map {
    current: (i32,i32),
    node_dict: HashMap<(i32,i32),Node<Scenario>>,
    x_range: (u16,u16),
    y_range: (u16,u16)
}

impl Map {
    pub fn new(x_range: (u16,u16),y_range:(u16,u16)) -> Map{
        let mut dict:HashMap<(i32,i32),Node<Scenario>> = HashMap::new();
        let (_,x_max) = x_range;
        let (_,y_max) = y_range;
        let node = Node::from_pos(Scenario::new(x_max, y_max, color::Rgb(0,0,0)),ORIGIN);
        dict.insert(ORIGIN, node);
        Map {
            current: ORIGIN,
            node_dict: dict,
            x_range: x_range,
            y_range: y_range
        }
    }

    pub fn current_scenario(&self) -> &Scenario {
        match self.node_dict.get(&self.current) {
            Some(node) => &node.object(),
            None => panic!("No existe escenario en {:?}",self.current)
        }
    }
}