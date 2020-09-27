use termion::color;

pub trait Printable {
    fn background_color(&self) -> color::Rgb;
    fn str_format(&self,x_abs:u16,y_abs:u16) -> String;
}