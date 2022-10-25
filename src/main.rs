extern crate termion;

use mine_sweeper::{Board, Position, Term};
use termion::event::{Key, Event, MouseEvent, MouseButton};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::fmt::write;
use std::io::{Write, stdout, stdin, Stdin};
use termion::{color, terminal_size};


fn main() {
    let mut stdin = stdin();
    let mut stdout = 
        termion::cursor::HideCursor::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()));

    game_loop(&mut stdin, &mut stdout, Board::new(4,4, 1).unwrap())
}

// fn start_menu(stdin: &mut Stdin, stdout: &mut Term) {
//     let menu_items: Vec<&str> = vec![
//         "Easy: 9x9 and 10 bombs",
//         "Medimum: 16x16 and 40 bombs",
//         "Hard: 30x16 and 99 bombs",
//         "Extreme: 24x30 and 180 bombs"
//     ];
//     fn print_menu(menu_items: Vec<&str>) {

//     }
//     for c in stdin.events() {
//         let evt = c.unwrap();
//         match evt {
//             Event::Key(Key::Char('q')) => break,
//             Event::Mouse(me) => {
//                 match me {
//                     MouseEvent::Press(MouseButton::Left, x, y) => {
//                         board.show_field(Position {row: ((y-1)) as usize, col: ((x-1)/2) as usize});
//                         board.print_board(stdout);
//                     },
//                     _ => (),
//                 }
//                 board.print_board(stdout);
//             }
//             _ => {}
//         }
//         stdout.flush().unwrap();
//     }
// }

fn game_loop(stdin: &mut Stdin, stdout: &mut Term, mut board: Board) {
    board.print_board(stdout);
    stdout.flush().unwrap();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(MouseButton::Left, x, y) => {
                        board.show_field(Position {row: ((y-1)) as usize, col: ((x-1)/2) as usize});
                        board.print_board(stdout);
                    },
                    _ => (),
                }
                board.print_board(stdout);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}