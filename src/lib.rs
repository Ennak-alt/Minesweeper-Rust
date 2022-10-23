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

#[derive(Debug, Clone)]
struct Field {
    Visibility: Visibility,
    FieldType: FieldType,
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
    fn get_field_val(&self, row: usize, col: usize) -> Option<FieldType> {
        if let Some(f) = self.get_field(row, col) {
            Some(f.FieldType)
        } else {
            None
        }
    }

    fn update(&mut self, row: usize, col: usize, new_field: Field) -> Result<(), &'static str> {
        if row > self.height-1 || col > self.width {
            return Err("Index out of bounds");
        }
        *(self.board.get_mut(row).unwrap().get_mut(col).unwrap()) = new_field;
        Ok(())
    }
    fn update_val(&mut self, row: usize, col: usize, val: FieldType) -> Result<(), &'static str> {
        self.update(
            row,
            col,
            Field {
                FieldType: val,
                ..self.get_field(row, col).unwrap()
            },
        )
    }
    fn update_vis(&mut self, row: usize, col: usize, vis: Visibility) -> Result<(), &'static str> {
        self.update(
            row,
            col,
            Field {
                Visibility: vis,
                ..self.get_field(row, col).unwrap()
            },
        )
    }

    pub fn show_field(&mut self, row: usize, col: usize) -> Option<FieldType> {
        self.update_vis(row, col, Visibility::Visible);
        self.get_field_val(row, col)
    }
    pub fn all_fields_visible(&mut self) {
        for rowi in 0..self.height {
            for coli in 0..self.width {
                self.update_vis(rowi, coli, Visibility::Visible);
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
                        Visibility: Visibility::Hidden,
                        FieldType: FieldType::SafeField(0)
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
            new_board.update_val(row_index, col_index, FieldType::BombField);
        }

        // Update non-bomb-fields with numbers of bombs around
        for row in 0..height {
            for col in 0..width {
                if let Some(FieldType::SafeField(_)) = new_board.get_field_val(row, col) {
                    let mut bombs_around = 0;
                    for i in -1..2 {
                        for j in -1..2 {
                            if i == 0 && j == 0 {
                                continue;
                            }
                            let row_index = row as i32 + i;
                            let col_index = col as i32 + j;
                            if row_index >= 0 && col_index >= 0 {
                                if let Some(FieldType::BombField) =
                                    new_board.get_field_val(row_index as usize, col_index as usize)
                                {
                                    bombs_around += 1;
                                }
                            }
                        }
                    }
                    if bombs_around != 0 {
                        new_board.update_val(row, col, FieldType::SafeField(bombs_around));
                    }
                }
            }
        }
        Ok(new_board)
    }
}
