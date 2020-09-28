use termion::color;

pub trait Printable {
    fn background_color(&self) -> color::Rgb;
    fn rel_pos(&self) -> (u16,u16);
    fn rel_pos_last(&self) -> (u16,u16);
    fn str_format(&self,x_abs:u16,y_abs:u16) -> String;
    fn del_format(&self,x_abs:u16,y_abs:u16,bg_default: color::Rgb) -> String;
}