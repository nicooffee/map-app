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


pub fn new() -> (Screen<RawTerminal<Stdout>>,Map) {
    let s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()), 20);
    let (main_w,main_h) = s.get_sizes(Panel::Main);
    let m = Map::new((main_w/2,main_w),(main_h/2,main_h));
    (s,m)
}


pub fn run(screen: Screen<RawTerminal<Stdout>>,map: Map) {
    let (tx_character,rx_character) = channel();
    //let (tx_info,rx_info) = channel();
    let (tx_exit,rx_exit) = channel();
    let map = Arc::new(Mutex::new(map));
    let screen = Arc::new(Mutex::new(screen));
    let thr_stdin   = thread::spawn(move || {run_stdin(tx_character,tx_exit)});
    let thr_main    = thread::spawn(move || {
        let clone_screen = Arc::clone(&screen);
        let clone_map = Arc::clone(&map);
        run_main(rx_exit,clone_screen,clone_map)
    
    });
    thr_stdin.join().unwrap();
    thr_main.join().unwrap();
}


fn run_stdin(
    tx_character: Sender<Option<Panel>>,
    tx_exit: Sender<bool>
){
    let stdin = async_stdin();
    let mut it = stdin.keys();
    loop {
        let b = it.next();
        if let Some(event) = b {
            match event.unwrap() {
                Key::Char('q') => {
                    tx_exit.send(true).unwrap();
                    break;
                },
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

fn run_info(){

}

fn run_main(
    rx_exit: Receiver<bool>,
    screen: Arc<Mutex<Screen<RawTerminal<Stdout>>>>,
    map: Arc<Mutex<Map>>
){
    let mut map = map.lock().unwrap();
    let mut screen = screen.lock().unwrap();
    loop{
        screen.initialize();
        screen.write_printable(1, 1, map.current_scenario());
        if let Ok(_) = rx_exit.try_recv(){break;};
        thread::sleep(time::Duration::from_micros(10000));
    }
}

fn run_character(){

}