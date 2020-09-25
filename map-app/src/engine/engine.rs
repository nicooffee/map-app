use std::{
    io::Write
};
use super::window::screen;


struct Engine<W: Write> {
    screen: screen::Screen<W>
}