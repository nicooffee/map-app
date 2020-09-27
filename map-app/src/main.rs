#[allow(dead_code)]
extern crate termion;
mod engine;
use engine::window::screen::Screen;
use engine::window::screen::Panel;
use engine::structure::scenario;
use engine::engine::{run,new};
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
    let (s,m) = new();
    run(s,m);
}
