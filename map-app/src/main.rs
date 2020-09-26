#[allow(dead_code)]
extern crate termion;
mod engine;
use engine::window::screen::Screen;
use engine::window::screen::Panel;
use engine::structure::scenario;
use engine::engine::Engine;
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
    let mut e = Engine::new();
    e.run();
}
