extern crate termion_ext;
use termion_ext::AdvWrite;
use crate::engine::traits::printable;
use std::io::{
    Write,
    Error
};
use termion::{
    screen::AlternateScreen,
    terminal_size,
    color,
    cursor,
    clear
};


pub struct Screen<W: Write> {
    window: AlternateScreen<W>,
    menu_w: u16,
    menu_h: u16,
    main_w: u16,
    main_h: u16
}

pub enum Panel {
    Menu,
    Main
}

impl<W:Write> Screen<W> {
    pub fn new(window: AlternateScreen<W>,menu_percent: u16) -> Screen<W>{
        let (width,height) = match terminal_size(){
            Ok((w,h)) => (w,h),
            Err(e) => panic!("Error al crear Screen: {}",e)
        };
        
        Screen {
            window: window,
            menu_w: (width * menu_percent / 100) - 1,
            menu_h: height -2,
            main_w: width - ((width * menu_percent / 100)) - 2,
            main_h: height - 2
        }
    }

    pub fn initialize(&mut self) {
        write!(self.window,"{}{}",clear::All,cursor::Hide).unwrap();
        self.window.w_box(1,1,self.w_width(),self.w_height(),None,None);
        self.window.w_line_v(self.border2(),2,self.menu_h,'|');
        self.window.flush().unwrap();
    }

    pub fn write_f(&mut self,panel: Panel,formatted_str: String) {
        writeln!(self.window,"{}{}{}",
            formatted_str,
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        
        ).unwrap();
    }

    pub fn write_printable<P: printable::Printable>(&mut self,x_rel:u16,y_rel:u16,ptbl_obj: &P) {
        let x_abs = self.x_rel_to_abs(x_rel, Panel::Main);
        let y_abs = self.y_rel_to_abs(y_rel, Panel::Main);
        self.write_f(Panel::Main, ptbl_obj.str_format(x_abs, y_abs));
    }
}

//Getter
impl<W: Write> Screen<W> {
    pub fn border1(&self) -> u16 {
        1
    }
    pub fn border2(&self) -> u16 {
        self.menu_w+1
    }
    pub fn border3(&self) -> u16 {
        self.menu_w+self.main_w+3
    }
    pub fn w_width(&self) -> u16 {
        self.menu_w+self.main_w+3
    }
    pub fn w_height(&self) -> u16 {
        self.menu_h + 2
    }
    
    pub fn get_sizes(&self,panel: Panel) -> (u16,u16) {
        match panel {
            Panel::Main => (self.main_w,self.main_h),
            Panel::Menu => (self.menu_w,self.menu_h)
        }
    }
    
    pub fn x_rel_to_abs(&self,x_rel:u16,panel: Panel) -> u16{
        match panel {
            Panel::Menu => x_rel + 1,
            Panel::Main => x_rel + self.menu_w + 1 
        }
    }
    pub fn y_rel_to_abs(&self,y_rel:u16,panel: Panel) -> u16{
        match panel {
            Panel::Menu => y_rel + 1,
            Panel::Main => y_rel + 1
        }
    }
}


impl<W: Write> Write for Screen<W> {
    fn write(&mut self,buf: &[u8]) -> Result<usize,Error> {
        self.window.write(buf)
    }

    fn flush(&mut self) -> Result<(),Error> {
        self.window.flush()
    }
}