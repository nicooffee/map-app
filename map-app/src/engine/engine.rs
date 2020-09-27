use std::{
    io::Stdout,
    io::stdout,
    thread,
    time,
    sync::{Arc, Mutex, mpsc::*}
};
use termion::{
    input::TermRead,
    event::Key,
    async_stdin,
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
        let s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()), 20);
        let (main_w,main_h) = s.get_sizes(Panel::Main);
        let m = Map::new((main_w/2,main_w),(main_h/2,main_h));
        Engine{
            screen: s,
            map: m
        }
    }


    pub fn run(&mut self) {
        let (tx_character,rx_character) = channel();
        //let (tx_info,rx_info) = channel();
        let stdin_thread = thread::spawn(move || {stdin_run(tx_character)});
        self.screen.initialize();
        self.screen.write_printable(1, 1, self.map.current_scenario());
        stdin_thread.join().unwrap();
    }
}

fn stdin_run(tx_character: Sender<Option<Panel>>){
    let stdin = async_stdin();
    let mut it = stdin.keys();
    loop {
        let b = it.next();
        if let Some(event) = b {
            match event.unwrap() {
                Key::Char('q') => break,
                Key::Char(c)   => println!("{}", c),
                Key::Alt(c)    => println!("Alt-{}", c),
                Key::Ctrl(c)   => println!("Ctrl-{}", c),
                Key::Left      => println!("<left>"),
                Key::Right     => println!("<right>"),
                Key::Up        => println!("<up>"),
                Key::Down      => println!("<down>"),
                _              => println!("Other")
            }
        }
        thread::sleep(time::Duration::from_micros(10000));
    }
}

fn info_run(){

}

fn main_run(){

}

fn character_run(){

}