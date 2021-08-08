mod ai;
mod fsm;
mod grid;

fn main() {
    let mut current_state = fsm::StateType::initial_state();

    while let Some(new_state) = current_state.to_next_state() {
        current_state = new_state;
    }
}
