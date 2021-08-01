mod grid;
mod minimax;

use crate::grid::{get_winner, Grid, Mark};
use crate::minimax::AlphaBetaPruning;
use std::io;
use std::thread::sleep;
use std::time::Duration;

const MAX_GRID_SIZE: usize = 50;

fn start_game() -> (Grid, AlphaBetaPruning, Mark) {
    let (mut grid, engine) = loop {
        match create_new_game() {
            Ok(t) => break t,
            Err(e) => println!("{}! 😡", e),
        }
    };

    let player_side = match engine.get_ai_side() {
        Mark::Cross => Mark::Nought,
        Mark::Nought => Mark::Cross,
    };
    if player_side == Mark::Nought {
        engine.try_make_move(&mut grid);
    }
    println!("{}", &grid);

    (grid, engine, player_side)
}

fn create_new_game() -> io::Result<(Grid, AlphaBetaPruning)> {
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
            side_str.trim().to_uppercase()
        };

        if side_str == "X" {
            Ok(Mark::Nought)
        } else if side_str == "O" {
            Ok(Mark::Cross)
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid side"))
        }
    }

    let ai_side = ai_side_prompt()?;
    println!("AI side is {:?}", ai_side);

    Ok((Grid::new(grid_size), AlphaBetaPruning::new(ai_side)))
}

fn play_one_round(grid: &mut Grid, engine: &AlphaBetaPruning, player_side: Mark) {
    loop {
        println!("Make your move!");
        match make_player_move(grid, player_side) {
            Ok(_) => break,
            Err(e) => println!("{}! 😡", e),
        }
    }
    println!("{}", grid);

    sleep(Duration::from_secs(1));
    if engine.try_make_move(grid) {
        println!("{}", grid);
    }
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

fn check_finished(grid: &Grid, player_side: Mark) -> bool {
    if let Some(winner) = get_winner(&grid) {
        if winner == player_side {
            println!("Congratulations, you won! 😱");
        } else {
            println!("Condolences, you lost 😰");
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
            answer.to_uppercase()
        };
        if answer.trim() == "Y" {
            Ok(true)
        } else if answer.trim() == "N" {
            Ok(false)
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Inappropriate answer",
            ))
        }
    }
    loop {
        match continue_prompt() {
            Ok(should_continue) => break should_continue,
            Err(e) => println!("{}! 😡", e),
        };
    }
}

fn main() {
    loop {
        let (mut grid, engine, player_side) = start_game();
        loop {
            play_one_round(&mut grid, &engine, player_side);
            if check_finished(&grid, player_side) {
                break;
            }
        }
        if !should_continue() {
            break;
        }
    }
}
