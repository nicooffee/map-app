#[allow(dead_code)]
extern crate termion;
mod window;
use window::screen::Screen;
use window::screen::Panel;
use std::{
    io::stdout,
    io::Write,
    thread,
    time
};
use termion::{
    screen::AlternateScreen,
    raw::IntoRawMode,
    terminal_size
};

fn main() {
    let mut s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()), 20);
    let (me_w,me_h,ma_w,ma_h) = s.get_sizes();
    let str1 = format!("me: {},{} ma: {},{}",me_w,me_h,ma_w,ma_h);
    s.write_f(Panel::Main,str1);
    s.initialize();
    thread::sleep(time::Duration::from_secs(2));
}
