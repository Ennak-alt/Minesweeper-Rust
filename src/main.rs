use mine_sweeper;
fn main() {
    let mut b = mine_sweeper::Board::new(3, 3, 1).unwrap();
    b.print_board();
    b.show_field(mine_sweeper::Position { row: 0, col: 0 });
    b.print_board();
    b.show_field(mine_sweeper::Position { row: 2, col: 2 });
    b.print_board();
}
