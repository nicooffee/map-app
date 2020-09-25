pub trait Printable {
    fn str_format(&self,x_abs:u16,y_abs:u16) -> String;
}