use termion::{
    color,
    cursor
};
use super::direction::Dir;
use crate::engine::{
    traits::printable
};
pub struct Character {
    pos: (i32,i32),
    sym: char,
    fg_color: color::Rgb,
    bg_color: color::Rgb
}


impl Character {
    pub fn new(pos: (i32,i32),sym: char,fg_color: color::Rgb,bg_color: color::Rgb) -> Character{
        Character {
            pos: pos,
            sym: sym,
            fg_color: fg_color,
            bg_color: bg_color
        }
    }

    pub fn move_chr(&mut self, dir: Dir) -> (i32,i32){
        self.pos = dir.of(self.pos);
        self.pos
    }


    pub fn pos(&self) -> (i32,i32){
        self.pos
    }
}

impl printable::Printable for Character {

    fn background_color(&self) -> color::Rgb {
        self.bg_color
    }

    fn str_format(&self,x_abs:u16,y_abs:u16) -> String {
        let mut str_fmt: String = String::from("");
        str_fmt.push_str(&format!("{}{}{}",
            color::Fg(self.fg_color),
            cursor::Goto(x_abs,y_abs),
            self.sym
        ));

        str_fmt
    }
}