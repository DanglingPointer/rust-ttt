mod grid;
mod minimax;

use crate::grid::{get_winner, Grid, Mark};
use crate::minimax::AlphaBetaPruning;

fn main() {
    println!("Hello, world!");

    let mut g = Grid::new(3);
    println!("after new: {:?}", g);

    g.set_at_pos(1, 1, Mark::Cross).unwrap_or_default();
    g.set_at_pos(0, 2, Mark::Nought).unwrap_or_default();
    println!("after setting: {:?}", g);
    println!("square at (1, 1) is: {:?}", g.get_at_pos(1, 1));

    let engine = AlphaBetaPruning::new(Mark::Cross);
    engine.try_make_move(&mut g);
    println!("after AI has done its move: {:?}", g);

    g.clear();
    println!("after clear: {:?}", g);
    println!("is grid full? {:?}", g.is_full());

    println!("and the WINNER is: {:?}", get_winner(&g));
}
