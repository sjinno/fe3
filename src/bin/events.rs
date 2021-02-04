use termion::event::Key;
use termion::input::TermRead;

enum Event {
    Input(Key),
}

fn main() -> ! {
    loop {
        let stdin = std::io::stdin();
        for c in stdin.keys() {
            let key = Event::Input(c.unwrap_or(Key::Null));
            match key {
                Event::Input(Key::Left) => println!("Left!"),
                _ => {}
            }
        }
    }
}
