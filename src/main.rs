extern crate termion;

use mine_sweeper::{Board, FieldType, Position, Term};
use std::collections::HashMap;
use std::io::{stdin, stdout, Stdin, Write};
use termion::event::{Event, Key, MouseButton, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;
use termion::terminal_size;


#[derive(Debug, Clone, Copy)]
enum MenuCommand {
    Play(usize, usize, usize),
    Quit,
}

type MenuItem = (&'static str, MenuCommand);

fn main() {
    let mut stdin = stdin();
    let mut stdout =
        termion::cursor::HideCursor::from(MouseTerminal::from(stdout().into_raw_mode().unwrap()));
    start_menu(&mut stdin, &mut stdout);
    //game_loop(&mut stdin, &mut stdout, Board::new(9, 9, 10).unwrap())
}

fn start_menu(stdin: &mut Stdin, stdout: &mut Term) {
    let s = [
        r#" __    __   __   __   __   ______   ______   __     __   ______   ______   ______  ______   ______   "#,
        r#"/\ "-./  \ /\ \ /\ "-.\ \ /\  ___\ /\  ___\ /\ \  _ \ \ /\  ___\ /\  ___\ /\  == \/\  ___\ /\  == \  "#,
        r#"\ \ \-./\ \\ \ \\ \ \-.  \\ \  __\ \ \___  \\ \ \/ ".\ \\ \  __\ \ \  __\ \ \  _-/\ \  __\ \ \  __<  "#,
        r#" \ \_\ \ \_\\ \_\\ \_\\"\_\\ \_____\\/\_____\\ \__/".~\_\\ \_____\\ \_____\\ \_\   \ \_____\\ \_\ \_\"#,
        r#"  \/_/  \/_/ \/_/ \/_/ \/_/ \/_____/ \/_____/ \/_/   \/_/ \/_____/ \/_____/ \/_/    \/_____/ \/_/ /_/"#,
    ];
    let menu_items: HashMap<u16, MenuItem> = HashMap::from([
        (6, ("Super easy 3x3 and 1 bomb", MenuCommand::Play(3, 3, 1))),
        (8, ("Easy: 9x9 and 10 bombs", MenuCommand::Play(9, 9, 10))),
        (
            9,
            ("Medimum: 16x16 and 40 bombs", MenuCommand::Play(16, 16, 40)),
        ),
        (
            10,
            ("Hard: 30x16 and 99 bombs", MenuCommand::Play(30, 16, 99)),
        ),
        (
            11,
            (
                "Extreme: 24x30 and 180 bombs",
                MenuCommand::Play(24, 30, 180),
            ),
        ),
        (12, ("Quit", MenuCommand::Quit)),
    ]);
    write!(
        stdout,
        "{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    let mut i = 1;
    for item in s {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(
                (terminal_size().unwrap().0 as u16 - item.len() as u16) / 2,
                i + (terminal_size().unwrap().1 as u16) / 3
            ),
            item
        )
        .unwrap();
        i += 1;
    }
    write!(stdout, "{}", termion::cursor::Goto(1, i)).unwrap();
    for item in &menu_items {
        let (row, (menustr, _)) = item;
        write!(
            stdout,
            "{}< {} >",
            termion::cursor::Goto(
                (terminal_size().unwrap().0 as u16 - menustr.len() as u16) / 2,
                row + (terminal_size().unwrap().1 as u16) / 3
            ),
            menustr
        )
        .unwrap();
    }
    write!(stdout, "{}", termion::cursor::Goto(1, 13)).unwrap();
    stdout.flush().unwrap();
    let mut n: MenuCommand = MenuCommand::Quit;
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(MouseButton::Left, _, y) => {
                    if let Some(h) = menu_items.get(&(y - (terminal_size().unwrap().1 as u16) / 3))
                    {
                        n = h.1;
                        break;
                    }    
                }
                _ => (),
            },
            _ => {}
        }
    }
    match n {
        MenuCommand::Play(width, height, bombs) => {
            game_loop(stdin, stdout, Board::new(width, height, bombs).unwrap())
        }
        MenuCommand::Quit => {}
    }
}

fn game_loop(stdin: &mut Stdin, stdout: &mut Term, mut board: Board) {
    board.print_board(stdout);
    stdout.flush().unwrap();
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(MouseButton::Right, x, y) => {
                        if let Some(flagged) = board.get_field_flagged(Position { 
                            row: (y - 1) as usize,
                            col: ((x - 1) / 2) as usize,
                        }) {
                            match board.update_field_flag(Position {
                                row: (y - 1) as usize,
                                col: ((x - 1) / 2) as usize,
                            }, !flagged) { 
                                _ => (), 
                            }
                        }
                    },
                    MouseEvent::Press(MouseButton::Left, x, y) => {
                        if let Some(field_type) = board.show_field(Position {
                            row: (y - 1) as usize,
                            col: ((x - 1) / 2) as usize,
                        }) {
                            if let FieldType::BombField = field_type {
                                board.all_fields_visible();
                                board.print_board(stdout);
                                write!(
                                    stdout,
                                    "{} You lost {}",
                                    board.fields_cleared, termion::cursor::Goto(1, board.height as u16 + 2)
                                )
                                .unwrap();
                                stdout.flush().unwrap();
                                break;
                            }
                            
                            if board.is_win() {
                                board.all_fields_visible();
                                board.print_board(stdout);
                                write!(
                                    stdout,
                                    "You Won {}",
                                    termion::cursor::Goto(1, board.height as u16 + 2)
                                )
                                .unwrap();
                                stdout.flush().unwrap();
                                break;
                            }
                            
                        }
                    }
                    _ => (),
                }
                board.print_board(stdout);
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
