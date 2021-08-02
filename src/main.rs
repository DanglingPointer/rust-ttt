use crate::gamestates::StateType;

mod gamestates;
mod grid;
mod minimax;

fn main() {
    let mut current_state = StateType::initial_state();

    while let Some(new_state) = current_state.to_next_state() {
        current_state = new_state;
    }
}
