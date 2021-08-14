use crate::ai;
use crate::grid::{get_winner, Grid, Mark};
use std::io;
use std::thread::sleep;
use std::time::Duration;

pub enum StateType {
    Startup(StartupStateData),
    PlayerTurn(PlayerTurnStateData),
    AiTurn(AiTurnStateData),
    OutcomeCheck(OutcomeCheckStateData),
}

impl StateType {
    pub fn initial_state() -> StateType {
        StateType::Startup(StartupStateData {})
    }
    pub fn to_next_state(self) -> Option<StateType> {
        match self {
            StateType::Startup(s) => next_state(s),
            StateType::PlayerTurn(s) => next_state(s),
            StateType::AiTurn(s) => next_state(s),
            StateType::OutcomeCheck(s) => next_state(s),
        }
    }
}

trait GameState {
    fn get_next_state(self) -> Option<StateType>;
}

fn next_state<T: GameState>(state: T) -> Option<StateType> {
    state.get_next_state()
}

pub struct StartupStateData;

impl GameState for StartupStateData {
    fn get_next_state(self) -> Option<StateType> {
        match create_new_game() {
            Ok((grid, engine)) => {
                println!("{}", &grid);
                if engine.get_ai_side() == Mark::Cross {
                    Some(StateType::AiTurn(AiTurnStateData { grid, engine }))
                } else {
                    Some(StateType::PlayerTurn(PlayerTurnStateData { grid, engine }))
                }
            }
            Err(e) => {
                println!("{}! 😡", e);
                Some(StateType::Startup(StartupStateData {}))
            }
        }
    }
}

pub struct PlayerTurnStateData {
    grid: Grid,
    engine: ai::AlphaBetaPruning,
}

impl GameState for PlayerTurnStateData {
    fn get_next_state(mut self) -> Option<StateType> {
        println!("Make your move!");
        let player_side = match self.engine.get_ai_side() {
            Mark::Cross => Mark::Nought,
            Mark::Nought => Mark::Cross,
        };
        match make_player_move(&mut self.grid, player_side) {
            Ok(_) => {
                println!("{}", self.grid);
                Some(StateType::AiTurn(AiTurnStateData {
                    grid: self.grid,
                    engine: self.engine,
                }))
            }
            Err(e) => {
                println!("{}! 😡", e);
                Some(StateType::PlayerTurn(PlayerTurnStateData {
                    grid: self.grid,
                    engine: self.engine,
                }))
            }
        }
    }
}

pub struct AiTurnStateData {
    grid: Grid,
    engine: ai::AlphaBetaPruning,
}

impl GameState for AiTurnStateData {
    fn get_next_state(mut self) -> Option<StateType> {
        sleep(Duration::from_secs(1));
        if self.engine.try_make_move(&mut self.grid) {
            println!("{}", self.grid);
        }
        Some(StateType::OutcomeCheck(OutcomeCheckStateData {
            grid: self.grid,
            engine: self.engine,
        }))
    }
}

pub struct OutcomeCheckStateData {
    grid: Grid,
    engine: ai::AlphaBetaPruning,
}

impl GameState for OutcomeCheckStateData {
    fn get_next_state(self) -> Option<StateType> {
        if check_finished(&self.grid, self.engine.get_ai_side()) {
            if should_continue() {
                Some(StateType::Startup(StartupStateData {}))
            } else {
                None
            }
        } else {
            Some(StateType::PlayerTurn(PlayerTurnStateData {
                grid: self.grid,
                engine: self.engine,
            }))
        }
    }
}

fn create_new_game() -> io::Result<(Grid, ai::AlphaBetaPruning)> {
    const MAX_GRID_SIZE: usize = 50;
    println!("\nWelcome to Tic-Tac-Toe 🙃");

    fn grid_size_prompt() -> io::Result<usize> {
        println!("Enter grid side length (e.g 3 for 3x3 grid):");
        let size_str: String = {
            let mut size_str = String::new();
            io::stdin().read_line(&mut size_str)?;
            size_str
                .trim()
                .chars()
                .take_while(|c| c.is_digit(10))
                .collect()
        };
        match size_str.parse::<usize>() {
            Ok(size_as_usize @ 2..=MAX_GRID_SIZE) => Ok(size_as_usize),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid grid size",
            )),
        }
    }

    let grid_size = grid_size_prompt()?;
    println!("Grid size is {}", grid_size);

    fn ai_side_prompt() -> io::Result<Mark> {
        println!("Choose side [X/O]:");
        let side_str = {
            let mut side_str = String::new();
            io::stdin().read_line(&mut side_str)?;
            side_str
        };

        match side_str.trim() {
            "X" | "x" => Ok(Mark::Nought),
            "O" | "o" => Ok(Mark::Cross),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid side")),
        }
    }

    let ai_side = ai_side_prompt()?;
    println!("AI side is {:?}", ai_side);

    Ok((Grid::new(grid_size), ai::AlphaBetaPruning::new(ai_side)))
}

fn make_player_move(grid: &mut Grid, player_side: Mark) -> io::Result<()> {
    fn index_prompt(grid_side_length: usize, name: &str) -> io::Result<usize> {
        let mut col_str = String::new();
        io::stdin().read_line(&mut col_str)?;
        let index = match col_str.trim().parse::<usize>() {
            Ok(index) => index,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Invalid {} index", name),
                ))
            }
        };
        if index < grid_side_length {
            Ok(index)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid {} index {}", name, index),
            ))
        }
    }
    println!("Enter column index for your next move:");
    let col = index_prompt(grid.get_side_length(), "column")?;

    println!("Enter row index for your next move:");
    let row = index_prompt(grid.get_side_length(), "row")?;

    match grid.set_at_pos(col, row, player_side) {
        Ok(()) => Ok(()),
        Err(mark) => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Square already contains {:?}", mark),
        )),
    }
}

fn check_finished(grid: &Grid, ai_side: Mark) -> bool {
    if let Some(winner) = get_winner(&grid) {
        if winner == ai_side {
            println!("Condolences, you lost 😰");
        } else {
            println!("Congratulations, you won! 😱");
        }
        true
    } else if grid.is_full() {
        println!("It's a draw! 😲");
        true
    } else {
        false
    }
}

fn should_continue() -> bool {
    fn continue_prompt() -> io::Result<bool> {
        println!("One more game? [Y/N]:");
        let answer = {
            let mut answer = String::new();
            io::stdin().read_line(&mut answer)?;
            answer
        };

        match answer.trim() {
            "Y" | "y" => Ok(true),
            "N" | "n" => Ok(false),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Inappropriate answer",
            )),
        }
    }
    loop {
        match continue_prompt() {
            Ok(should_continue) => break should_continue,
            Err(e) => println!("{}! 😡", e),
        };
    }
}
