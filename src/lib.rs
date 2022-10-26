use rand::Rng;
use std::io::{Stdout, Write};
use termion::{cursor::HideCursor, input::MouseTerminal, raw::RawTerminal};

pub type Term = HideCursor<MouseTerminal<RawTerminal<Stdout>>>;

#[derive(Clone, Debug)]
pub enum FieldType {
    BombField,
    SafeField(i64),
}

#[derive(Clone, Debug)]
enum Visibility {
    Hidden,
    Visible,
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone)]
struct Field {
    visibility: Visibility,
    field_type: FieldType,
}

#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Field>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    fn get_field(&self, pos: Position) -> Option<Field> {
        if let Some(r) = self.board.get(pos.row) {
            if let Some(f) = r.get(pos.col) {
                return Some(f.clone());
            }
        }
        None
    }

    fn get_field_vis(&self, pos: Position) -> Option<Visibility> {
        if let Some(f) = self.get_field(pos) {
            Some(f.visibility)
        } else {
            None
        }
    }

    fn get_field_type(&self, pos: Position) -> Option<FieldType> {
        if let Some(f) = self.get_field(pos) {
            Some(f.field_type)
        } else {
            None
        }
    }

    fn update_field(&mut self, pos: Position, new_field: Field) -> Result<(), &'static str> {
        *(self
            .board
            .get_mut(pos.row)
            .ok_or("Row index out of bounds.")?
            .get_mut(pos.col)
            .ok_or("Collumn index out of bounds.")?) = new_field;
        Ok(())
    }

    fn update_field_vis(&mut self, pos: Position, vis: Visibility) -> Result<(), &'static str> {
        self.update_field(
            pos,
            Field {
                visibility: vis,
                ..self.get_field(pos).ok_or("Out of bounds.")?
            },
        )
    }

    fn update_field_type(&mut self, pos: Position, val: FieldType) -> Result<(), &'static str> {
        self.update_field(
            pos,
            Field {
                field_type: val,
                ..self.get_field(pos).ok_or("Out of bounds")?
            },
        )
    }

    fn get_fields_around(&mut self, pos: Position) -> Vec<Position> {
        let mut positions_around: Vec<Position> = Vec::new();
        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }
                let row_index = pos.row as i32 + i;
                let col_index = pos.col as i32 + j;
                if row_index >= 0
                    && col_index >= 0
                    && row_index < self.height as i32
                    && col_index < self.width as i32
                {
                    positions_around.push(Position {
                        row: row_index as usize,
                        col: col_index as usize,
                    });
                }
            }
        }
        positions_around
    }

    pub fn show_field(&mut self, pos: Position) -> Option<FieldType> {
        self.update_field_vis(pos, Visibility::Visible).ok()?;
        let field_type = self.get_field_type(pos);
        match field_type {
            Some(FieldType::SafeField(0)) => {
                fn show_zero_fields(board: &mut Board, pos: Position) -> Result<(), &'static str> {
                    match board.get_field_type(pos) {
                        Some(FieldType::SafeField(0)) => {
                            board.update_field_vis(pos, Visibility::Visible)?;
                            for a_pos in board.get_fields_around(pos) {
                                if let Some(Visibility::Hidden) = board.get_field_vis(a_pos) {
                                    show_zero_fields(board, a_pos)?;
                                }
                            }
                        }
                        Some(FieldType::SafeField(_)) => {
                            board.update_field_vis(pos, Visibility::Visible).unwrap();
                        }
                        _ => {}
                    }
                    Ok(())
                }
                show_zero_fields(self, pos).ok()?;
            }
            Some(FieldType::BombField) | Some(FieldType::SafeField(_)) => {
                self.update_field_vis(pos, Visibility::Visible).unwrap();
            }
            None => {}
        }
        field_type
    }

    pub fn all_fields_visible(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                self.update_field_vis(Position { row: row, col: col }, Visibility::Visible)
                    .unwrap();
            }
        }
    }

    pub fn new(width: usize, height: usize, bombs: usize) -> Result<Self, &'static str> {
        if width * height < bombs {
            return Err("Amount of bombs was bigger than amount of fields.");
        }
        let mut new_board: Board = Board {
            board: vec![
                vec![
                    Field {
                        visibility: Visibility::Hidden,
                        field_type: FieldType::SafeField(0)
                    };
                    width
                ];
                height
            ],
            width: width,
            height: height,
        };
        let mut rng = rand::thread_rng();

        // Create bomb-fields
        for _ in 0..bombs {
            let mut pos: Vec<Position> = Vec::new();
            for row in 0..height {
                for col in 0..width {
                    pos.push(Position { row: row, col: col });
                }
            }
            let pos = pos.remove(rng.gen_range(0..pos.len()));
            new_board
                .update_field_type(pos, FieldType::BombField)
                .unwrap();
        }

        // update_field non-bomb-fields with numbers of bombs around
        for row in 0..height {
            for col in 0..width {
                let pos = Position { row: row, col: col };
                if let Some(FieldType::SafeField(_)) = new_board.get_field_type(pos) {
                    let mut bombs_around = 0;
                    for pos in new_board.get_fields_around(pos) {
                        if let Some(FieldType::BombField) = new_board.get_field_type(pos) {
                            bombs_around += 1;
                        }
                    }
                    if bombs_around != 0 {
                        new_board
                            .update_field_type(pos, FieldType::SafeField(bombs_around))
                            .unwrap();
                    }
                }
            }
        }
        Ok(new_board)
    }

    pub fn print_board(&self, stdout: &mut Term) {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1)
        )
        .unwrap();
        for row in 0..self.height {
            for col in 0..self.width {
                match self.get_field(Position { row: row, col: col }).unwrap() {
                    Field {
                        visibility: Visibility::Hidden,
                        field_type: _,
                    } => write!(stdout, "⬜️").unwrap(),
                    Field {
                        visibility: _,
                        field_type: FieldType::BombField,
                    } => write!(stdout, "💣").unwrap(),
                    Field {
                        visibility: _,
                        field_type: FieldType::SafeField(n),
                    } => write!(stdout, " {}", n).unwrap(),
                }
            }
            write!(stdout, "{}", termion::cursor::Goto(1, row as u16 + 2)).unwrap();
        }
    }
}
