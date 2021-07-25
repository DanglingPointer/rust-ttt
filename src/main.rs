mod field;
mod minimax;

use crate::field::{get_winner, Field, Mark};

fn main() {
    println!("Hello, world!");

    let mut f = Field::new();
    println!("after new: {:?}", f);

    f.set_at(1, 1, Mark::Cross).unwrap_or_default();
    f.set_at(0, 2, Mark::Nought).unwrap_or_default();
    println!("after setting: {:?}", f);
    println!("square at (1, 1) is: {:?}", f.get_at(1, 1));

    f.clear();
    println!("after clear: {:?}", f);
    println!("is field full? {:?}", f.is_full());

    println!("and the WINNER is: {:?}", get_winner(&f));
}
