use rand::Rng;

#[derive(Clone, Debug)]
enum FieldVal {
    BombField,
    SafeField(i64),
}

#[derive(Clone, Debug)]
enum Visibility {
    Hidden,
    Visible,
}

#[derive(Debug, Clone)]
struct Field {
    Visibility: Visibility,
    FieldVal: FieldVal,
}

#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Field>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    fn get_field(&self, row: usize, col: usize) -> Option<Field> {
        if let Some(r) = self.board.get(row) {
            if let Some(f) = r.get(col) {
                return Some(f.clone());
            }
        }
        None
    }
    fn get_field_vis(&self, row: usize, col: usize) -> Option<Visibility> {
        if let Some(f) = self.get_field(row, col) {
            Some(f.Visibility)
        } else {
            None
        }
    }
    fn get_field_val(&self, row: usize, col: usize) -> Option<FieldVal> {
        if let Some(f) = self.get_field(row, col) {
            Some(f.FieldVal)
        } else {
            None
        }
    }

    fn update(&mut self, row: usize, col: usize, new_field: Field) {
        *(self.board.get_mut(row).unwrap().get_mut(col).unwrap()) = new_field;
    }
    fn update_val(&mut self, row: usize, col: usize, val: FieldVal) {
        self.update(
            row,
            col,
            Field {
                FieldVal: val,
                ..self.get_field(row, col).unwrap()
            },
        );
    }
    fn update_vis(&mut self, row: usize, col: usize, vis: Visibility) {
        self.update(
            row,
            col,
            Field {
                Visibility: vis,
                ..self.get_field(row, col).unwrap()
            },
        );
    }

    pub fn new(width: usize, height: usize, bombs: usize) -> Result<Self, &'static str> {
        if width * height < bombs {
            return Err("Amount of bombs was bigger than amount of fields.");
        }
        let mut new_board: Board = Board {
            board: vec![
                vec![
                    Field {
                        Visibility: Visibility::Hidden,
                        FieldVal: FieldVal::SafeField(0)
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
            new_board.update_val(row_index, col_index, FieldVal::BombField);
        }

        // Update non-bomb-fields with numbers of bombs around
        for row in 0..height {
            for col in 0..width {
                if let Some(FieldVal::SafeField(_)) = new_board.get_field_val(row, col) {
                    let mut bombs_around = 0;
                    for i in -1..2 {
                        for j in -1..2 {
                            if i == 0 && j == 0 {
                                continue;
                            }
                            let row_index = row as i32 + i;
                            let col_index = col as i32 + j;
                            if row_index >= 0 && col_index >= 0 {
                                if let Some(FieldVal::BombField) =
                                    new_board.get_field_val(row_index as usize, col_index as usize)
                                {
                                    bombs_around += 1;
                                }
                            }
                        }
                    }
                    if bombs_around != 0 {
                        new_board.update_val(row, col, FieldVal::SafeField(bombs_around));
                    }
                }
            }
        }
        Ok(new_board)
    }
}
