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
const DELAY: u64 = 5000;

pub fn new() -> (Screen<RawTerminal<Stdout>>,Map,Character) {
    let s_bg = color::Rgb(0,51,51);
    let s = Screen::new(AlternateScreen::from(stdout().into_raw_mode().unwrap()),s_bg, 20);
    let (main_w,main_h) = s.get_sizes(Panel::Main);
    let m = Map::new((main_w/2,main_w),(main_h/2,main_h));
    let c = Character::new((1,1), '*', color::Rgb(200,200,200),m.current_scenario().background_color());
    (s,m,c)
}


pub fn run(screen: Screen<RawTerminal<Stdout>>,map: Map,character: Character) {
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
    let character   = Arc::new(Mutex::new(character));
    let cl_character_1 = Arc::clone(&character);
    let cl_character_2 = Arc::clone(&character);
    let thr_stdin   = thread::spawn(move || {run_stdin(tx_character,send_closure)});
    let thr_main    = thread::spawn(move || {
        let cl_screen = Arc::clone(&screen);
        let cl_map = Arc::clone(&map);
        run_main(rx_exit_main,cl_screen,cl_map,cl_character_1);
    
    });
    let thr_char    = thread::spawn(move ||{
        run_character(rx_exit_char,cl_character_2,rx_character);
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
                Key::Char(c)   => println!("{}", c),
                Key::Alt(c)    => println!("Alt-{}", c),
                Key::Ctrl(c)   => println!("Ctrl-{}", c),
                Key::Up        => tx_character.send(Dir::Up).unwrap(),
                Key::Down      => tx_character.send(Dir::Down).unwrap(),
                Key::Left      => tx_character.send(Dir::Left).unwrap(),
                Key::Right     => tx_character.send(Dir::Right).unwrap(),
                _              => println!("Other")
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
    map: Arc<Mutex<Map>>,
    character: Arc<Mutex<Character>>
){
    {
        let mut screen = screen.lock().unwrap();
        screen.initialize();
        screen.write_menu(TextLine::TITLE,"hola mundo");
    }
    loop{
        {
            let mut screen = screen.lock().unwrap();
            {
                let map = map.lock().unwrap();
                screen.write_printable(1, 1, map.current_scenario());
            }
            {
                let character = character.lock().unwrap();
                let (x,y) = character.pos();
                //Fix this
                screen.write_printable(x as u16,y as u16,&*character);
            }
            screen.flush().unwrap();
        }
        if let Ok(_) = rx_exit.try_recv(){break;};
        thread::sleep(time::Duration::from_micros(DELAY));
    }
}

fn run_character(
    rx_exit: Receiver<bool>,
    character: Arc<Mutex<Character>>,
    rx_character: Receiver<Dir>
){
    loop {
        let dir = rx_character.recv().unwrap();
        {
            let mut character = character.lock().unwrap();
            character.move_chr(dir);
        }
        if let Ok(_) = rx_exit.try_recv(){break;};
    }
}