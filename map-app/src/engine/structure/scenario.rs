use crate::engine::{
    traits::printable
};
use termion::{
    color,
    cursor
};

pub struct Scenario{
    rel_pos: (u16,u16),
    width: u16,
    height: u16,
    background: color::Rgb
}

impl Scenario {
    pub fn new(rel_pos: (u16,u16),width:u16,height:u16, bg: color::Rgb) -> Scenario{
        if width < 1 || height < 1{
            panic!("Las dimensiones deben ser positivas! w:{} h:{}",width,height);
        }
        Scenario{rel_pos:rel_pos,width:width,height:height,background:bg}
    }

}

impl printable::Printable for Scenario {
    fn background_color(&self) -> color::Rgb{
        self.background
    }
    fn rel_pos(&self) -> (u16,u16){
        self.rel_pos
    }
    fn rel_pos_last(&self) -> (u16,u16){
        self.rel_pos
    }
    fn str_format(&self,x_abs:u16,y_abs:u16) -> String {
        let mut str_fmt: String = String::from("");
        str_fmt.push_str(&format!("{}",color::Bg(self.background)));
        for i in 0..(self.height) {
            str_fmt.push_str(&format!("{}",cursor::Goto(x_abs,y_abs+i)));
            for _ in 0..self.width{
                str_fmt.push_str(" ");
            }
        }
        str_fmt
    }
    fn del_format(&self,x_abs:u16,y_abs:u16,bg_default: color::Rgb) -> String{
        String::from("")
    }
}