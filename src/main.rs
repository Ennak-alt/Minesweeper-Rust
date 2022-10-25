extern crate termion;

use mine_sweeper::{Board, Position};
use termion::event::{Key, Event, MouseEvent, MouseButton};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::fmt::write;
use std::io::{Write, stdout, stdin};
use termion::{color, terminal_size};


fn main() {
    let stdin = stdin();
    let mut stdout = 
        termion::cursor::HideCursor::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
    let mut board = Board::new(9,9, 10).unwrap();
    board.print_board(&mut stdout);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(MouseButton::Left, x, y) => {
                        board.show_field(Position {row: ((y-1)) as usize, col: ((x-1)/2) as usize});
                        board.print_board(&mut stdout);
                        write!(stdout, "{} {} ", x, y).unwrap();
                        write!(stdout, "{} {}", (x-1)/2, (y-1)).unwrap();
                    },
                    _ => (),
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}