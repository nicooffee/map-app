use std::{
    io::Stdout,
    io::stdout,
    io::Write,
    thread,
    time
};
use termion::{
    event::Key,
    raw::IntoRawMode,
    input::TermRead
};

fn  main() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin();
    let mut it = stdin.keys();
    let mut c = 0;
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
                _              => println!("Other"),
                _ => {}
            }
        }
        /*print!("\tHan pasado {} iteraciones.\r",c);
        stdout.flush().unwrap();
        c = c + 1;*/
        thread::sleep(time::Duration::from_micros(10000));
    }
}