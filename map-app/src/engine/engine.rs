use std::{
    io::Stdout,
    io::stdout,
    io::Write,
    thread,
    time
};
use termion::{
    raw::RawTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};
use super::window::screen::{
    Screen,
    Panel
};
use super::structure::map::Map;

pub struct Engine {
    screen: Screen<RawTerminal<Stdout>>,
    map: Map
}

impl Engine {
    pub fn new() -> Engine {
        let mut s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()), 20);
        let (main_w,main_h) = s.get_sizes(Panel::Main);
        let mut m = Map::new((main_w/2,main_w),(main_h/2,main_h));
        Engine{
            screen: s,
            map: m
        }
    }
    pub fn run(&mut self) {
        self.screen.initialize();
        self.screen.write_printable(1, 1, self.map.current_scenario());
        self.screen.flush().unwrap();
        thread::sleep(time::Duration::from_secs(10));
    }
}