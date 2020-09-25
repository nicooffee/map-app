extern crate termion_ext;

use std::io::{
    Write,
    Error
};
use termion_ext::AdvWrite;
use termion::{
    screen::AlternateScreen,
    terminal_size
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

    pub fn get_sizes(&self) -> (u16,u16,u16,u16) {
        (self.menu_w,self.menu_h,self.main_w,self.main_h)
    }


    pub fn write_f(&mut self,panel: Panel,formatted_str: String) {
        writeln!(self.window,"{}",formatted_str).unwrap();
    }

    pub fn initialize(&mut self) {
        self.window.w_box(1,1,self.w_width(),self.w_height(),None,None);
        self.window.flush().unwrap();
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
}


impl<W: Write> Write for Screen<W> {
    fn write(&mut self,buf: &[u8]) -> Result<usize,Error> {
        self.window.write(buf)
    }

    fn flush(&mut self) -> Result<(),Error> {
        self.window.flush()
    }
}