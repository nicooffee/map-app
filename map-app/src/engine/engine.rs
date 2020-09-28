use std::{
    io::Stdout,
    io::stdout,
    io::Write,
    thread,
    time,
    sync::{Arc, Mutex, mpsc::*}
};
use termion::{
    input::TermRead,
    event::Key,
    color,
    async_stdin,
    raw::RawTerminal,
    raw::IntoRawMode,
    screen::AlternateScreen
};
use super::window::screen::{
    Screen,
    Panel,
    TextLine
};
use super::structure::map::Map;
use super::structure::direction::Dir;
use super::structure::character::Character;
use crate::engine::traits::printable::Printable;
const DELAY: u64 = 10000;

pub fn new() -> (Screen<RawTerminal<Stdout>>,Map) {
    let s_bg = color::Rgb(0,51,51);
    let s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()),s_bg, 20);
    let (main_w,main_h) = s.get_sizes(Panel::Main);
    let m = Map::new((main_w/2,main_w),(main_h/2,main_h));
    (s,m)
}


pub fn run(screen: Screen<RawTerminal<Stdout>>,map: Map) {
    let (tx_character,rx_character) = channel();
    //let (tx_info,rx_info) = channel();
    let (tx_exit_main,rx_exit_main) = channel();
    let (tx_exit_info,rx_exit_info) = channel();
    let (tx_exit_char,rx_exit_char) = channel();
    let send_closure = move || {
        tx_exit_main.send(true).unwrap();
        tx_exit_info.send(true).unwrap();
        tx_exit_char.send(true).unwrap();
    };
    let map         = Arc::new(Mutex::new(map));
    let screen      = Arc::new(Mutex::new(screen));
    let cl_map_1    = Arc::clone(&map);
    let cl_map_2    = Arc::clone(&map);
    let thr_stdin   = thread::spawn(move || {run_stdin(tx_character,send_closure)});
    let thr_main    = thread::spawn(move || {
        let cl_screen = Arc::clone(&screen);
        run_main(rx_exit_main,cl_screen,cl_map_1);
    
    });
    let thr_char    = thread::spawn(move ||{
        run_character(rx_exit_char,cl_map_2,rx_character);
    });

    thr_stdin.join().unwrap();
    thr_main.join().unwrap();
    thr_char.join().unwrap();

}


fn run_stdin<T: Fn() -> ()>(
    tx_character: Sender<Dir>,
    tx_exit: T
){
    let stdin = async_stdin();
    let mut it = stdin.keys();
    loop {
        let b = it.next();
        if let Some(event) = b {
            match event.unwrap() {
                Key::Char('q') => {
                    tx_exit();
                    break;
                },
                Key::Up        => tx_character.send(Dir::Up).unwrap(),
                Key::Down      => tx_character.send(Dir::Down).unwrap(),
                Key::Left      => tx_character.send(Dir::Left).unwrap(),
                Key::Right     => tx_character.send(Dir::Right).unwrap(),
                _              => {}
            }
        }
        thread::sleep(time::Duration::from_micros(DELAY/2));
    }
}

fn run_info(){

}

fn run_main(
    rx_exit: Receiver<bool>,
    screen: Arc<Mutex<Screen<RawTerminal<Stdout>>>>,
    map: Arc<Mutex<Map>>
){
    let bg_default: color::Rgb;
    {
        let mut screen = screen.lock().unwrap();
        screen.initialize();
        screen.write_menu(TextLine::TITLE,"hola mundo");
        let map = map.lock().unwrap();
        bg_default = map.current_scenario().background_color();
        screen.write_printable(map.current_scenario(),bg_default);
        screen.flush().unwrap();
    }
    loop{
        {
            let mut screen = screen.lock().unwrap();
            {
                //Fix this
                screen.write_printable(map.lock().unwrap().character(),bg_default);
            }
            screen.flush().unwrap();
        }
        if let Ok(_) = rx_exit.try_recv(){break;};
        thread::sleep(time::Duration::from_micros(DELAY));
    }
}

fn run_character(
    rx_exit: Receiver<bool>,
    map: Arc<Mutex<Map>>,
    rx_character: Receiver<Dir>
){
    loop {
        {
            let mut map = map.lock().unwrap();
            if let Ok(dir) = rx_character.try_recv(){
                map.move_chr(dir);
            }
        }
        thread::sleep(time::Duration::from_micros(DELAY));
        if let Ok(_) = rx_exit.try_recv(){break;};
    }
}

/*try_recv
loop {
    {
        let mut character = character.lock().unwrap();
        if let Ok(dir) = rx_character.try_recv(){
            character.move_chr(dir);
        }
    }
    thread::sleep(time::Duration::from_micros(DELAY));
    if let Ok(_) = rx_exit.try_recv(){break;};
}
*/
/*recv
loop {
    let dir = rx_character.recv().unwrap();
    {
        let mut character = character.lock().unwrap();
        character.move_chr(dir);
    }
    thread::sleep(time::Duration::from_micros(DELAY));
    if let Ok(_) = rx_exit.try_recv(){break;};
}
*/