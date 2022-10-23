use rand::Rng;

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
        if pos.row > self.height - 1 || pos.col > self.width {
            return Err("Index out of bounds");
        }
        *(self
            .board
            .get_mut(pos.row)
            .unwrap()
            .get_mut(pos.col)
            .unwrap()) = new_field;
        Ok(())
    }

    fn update_field_vis(&mut self, pos: Position, vis: Visibility) -> Result<(), &'static str> {
        self.update_field(
            pos,
            Field {
                visibility: vis,
                ..self.get_field(pos).unwrap()
            },
        )
    }

    fn update_field_type(&mut self, pos: Position, val: FieldType) -> Result<(), &'static str> {
        self.update_field(
            pos,
            Field {
                field_type: val,
                ..self.get_field(pos).unwrap()
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
        self.update_field_vis(pos, Visibility::Visible).unwrap();
        let field_type = self.get_field_type(pos);
        match field_type {
            Some(FieldType::SafeField(0)) => {
                fn show_zero_fields(board: &mut Board, pos: Position) -> () {
                    if let Some(FieldType::SafeField(0)) = board.get_field_type(pos) {
                        board.update_field_vis(pos, Visibility::Visible).unwrap();
                        for a_pos in board.get_fields_around(pos) {
                            if let Some(Visibility::Hidden) = board.get_field_vis(a_pos) {
                                show_zero_fields(board, a_pos);
                            }
                        }
                    }
                }
                show_zero_fields(self, pos);
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

    pub fn print_board(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let t = self.get_field(Position { row: row, col: col }).unwrap();
                print!("({:?} ", t.visibility);
                print!("{:?}) | ", t.field_type);
            }
            println!();
        }
        println!();
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
        let mut row_indices: Vec<usize> = (0..height).collect();
        let mut col_indices: Vec<usize> = (0..width).collect();
        let mut rng = rand::thread_rng();

        // Create bomb-fields
        for _ in 0..bombs {
            let row_index = row_indices.remove(rng.gen_range(0..row_indices.len()));
            let col_index = col_indices.remove(rng.gen_range(0..col_indices.len()));
            new_board
                .update_field_type(
                    Position {
                        row: row_index,
                        col: col_index,
                    },
                    FieldType::BombField,
                )
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
}
