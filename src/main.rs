use mine_sweeper;
fn main() {
    let mut b = mine_sweeper::Board::new(2, 2, 2).unwrap();
    println!("{:?}", b);
    b.all_fields_visible();
    println!("{:?}", b); 
}
