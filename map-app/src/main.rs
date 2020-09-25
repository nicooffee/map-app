#[allow(dead_code)]
extern crate termion;
mod engine;
use engine::window::screen::Screen;
use engine::window::screen::Panel;
use engine::structure::scenario;
use std::{
    io::stdout,
    io::Write,
    thread,
    time
};
use termion::{
    screen::AlternateScreen,
    raw::IntoRawMode,
    terminal_size,
    color
};

fn main() {
    let mut s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()), 20);
    let (me_w,me_h,ma_w,ma_h) = s.get_sizes();
    s.initialize();
    let scene = scenario::Scenario::new(5, 5, color::Rgb(255,255,255));
    s.write_printable(1, 1, scene);
    thread::sleep(time::Duration::from_secs(2));
}
