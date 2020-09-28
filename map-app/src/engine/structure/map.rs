mod node;
use node::Node;
use super::scenario::Scenario;
use super::character::Character;
use super::direction::Dir;
use super::super::traits::printable::Printable;
use std::collections::HashMap;
use termion::{
    color
};
const ORIGIN: (i32,i32) = (0,0);

pub struct Map {
    current: (i32,i32),
    node_dict: HashMap<(i32,i32),Node<Scenario>>,
    x_range: (u16,u16),
    y_range: (u16,u16),
    character: Character
}

impl Map {
    pub fn new(x_range: (u16,u16),y_range:(u16,u16)) -> Map{
        let mut dict:HashMap<(i32,i32),Node<Scenario>> = HashMap::new();
        let (x_min,x_max) = x_range;
        let (y_min,y_max) = y_range;
        let node = Node::from_pos(Scenario::new((1,1),x_max, y_max, color::Rgb(0,0,0)),ORIGIN);
        let c = Character::new((1,1), '*', color::Rgb(200,200,200),*&node.object().background_color());
        dict.insert(ORIGIN, node);
        Map {
            current: ORIGIN,
            node_dict: dict,
            x_range: x_range,
            y_range: y_range,
            character: c
        }
    }

    pub fn current_scenario(&self) -> &Scenario {
        match self.node_dict.get(&self.current) {
            Some(node) => &node.object(),
            None => panic!("No existe escenario en {:?}",self.current)
        }
    }


    pub fn move_chr(&mut self, dir: Dir) -> (u16,u16){
        self.character.move_chr(dir)
    }


    pub fn pos_next_chr(&self,dir: Dir) -> (u16,u16) {
        self.character.pos_next(dir)
    }

    pub fn character(&self) -> &Character {
        &self.character
    }
}