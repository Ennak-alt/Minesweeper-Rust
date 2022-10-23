use mine_sweeper;
fn main() {
    let b = mine_sweeper::Board::new(4, 3, 2).unwrap();
    println!("{:?}", b);
}
