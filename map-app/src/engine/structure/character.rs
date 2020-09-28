use termion::{
    color,
    cursor
};
use super::direction::Dir;
use crate::engine::{
    traits::printable
};
pub struct Character {
    pos: (u16,u16),
    pos_last: (u16,u16),
    sym: char,
    fg_color: color::Rgb,
    bg_color: color::Rgb
}


impl Character {
    pub fn new(pos: (u16,u16),sym: char,fg_color: color::Rgb,bg_color: color::Rgb) -> Character{
        Character {
            pos: pos,
            pos_last: pos,
            sym: sym,
            fg_color: fg_color,
            bg_color: bg_color
        }
    }

    pub fn move_chr(&mut self, dir: Dir) -> (u16,u16){
        self.pos_last = self.pos;
        self.pos = dir.of_u16(self.pos);
        self.pos
    }


    pub fn pos_next(&self,dir: Dir) -> (u16,u16) {
        dir.of_u16(self.pos)
    }
}

impl printable::Printable for Character {

    fn background_color(&self) -> color::Rgb {
        self.bg_color
    }

    fn rel_pos(&self) -> (u16,u16){
        self.pos
    }
    fn rel_pos_last(&self) -> (u16,u16){
        self.pos_last
    }

    fn str_format(&self,x_abs:u16,y_abs:u16) -> String {
        let mut str_fmt: String = String::from("");
        str_fmt.push_str(&format!("{}{}{}{}",
            color::Bg(self.background_color()),
            color::Fg(self.fg_color),
            cursor::Goto(x_abs,y_abs),
            self.sym
        ));

        str_fmt
    }

    fn del_format(&self,x_abs:u16,y_abs:u16,bg_default: color::Rgb) -> String {
        let mut str_fmt: String = String::from("");
        str_fmt.push_str(&format!("{}{}{} ",
            color::Bg(bg_default),
            color::Fg(self.fg_color),
            cursor::Goto(x_abs,y_abs)
        ));
        str_fmt
    }
}