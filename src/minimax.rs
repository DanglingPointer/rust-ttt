use crate::grid::{get_winner, Grid, Mark};
use std::cmp::{max, min};
use std::io;
use std::io::Write;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

pub struct AlphaBetaPruning {
    max_side: Mark,
    min_side: Mark,
}

impl AlphaBetaPruning {
    pub fn new(ai_side: Mark) -> AlphaBetaPruning {
        AlphaBetaPruning {
            max_side: ai_side,
            min_side: match ai_side {
                Mark::Nought => Mark::Cross,
                Mark::Cross => Mark::Nought,
            },
        }
    }

    pub fn get_ai_side(&self) -> Mark {
        self.max_side
    }

    pub fn try_make_move(&self, grid: &mut Grid) -> bool {
        if get_winner(grid).is_some() {
            return false;
        }

        let mut alpha = Outcome::Loss;
        let beta = Outcome::Win;

        let mut best_outcome = Outcome::Loss; // worst outcome
        let mut best_move: Option<Move> = None;
        let mut last_move: Option<Move> = None;

        for ind in 0..grid.get_size() {
            let next_move = Move {
                what: self.max_side,
                ind,
            };
            if let Some(mover) = RevertingMoveMaker::from_move(grid, next_move) {
                println!("\rchecking move alternative {}...", ind);
                last_move = Some(mover.get_move());
                let outcome = self.minimizing_side(mover.grid, alpha, beta, 1);
                if outcome > best_outcome {
                    best_outcome = outcome;
                    best_move = Some(mover.get_move());
                }
                if best_outcome >= beta {
                    break;
                }
                alpha = max(alpha, best_outcome);
            }
        }

        println!("\r{:<width$}", "DONE", width = grid.get_size());
        if let Some(bm) = best_move {
            PersistentMoveMaker::from_move(grid, bm);
            true
        } else if let Some(lm) = last_move {
            PersistentMoveMaker::from_move(grid, lm);
            true
        } else {
            false
        }
    }

    fn maximizing_side(
        &self,
        grid: &mut Grid,
        mut alpha: Outcome,
        beta: Outcome,
        depth: usize,
    ) -> Outcome {
        if let Some(outcome) = self.check_finished(grid) {
            return outcome;
        }

        let mut best_outcome = Outcome::Loss; // worst outcome

        for ind in 0..grid.get_size() {
            let next_move = Move {
                what: self.max_side,
                ind,
            };
            if let Some(mover) = RevertingMoveMaker::from_move(grid, next_move) {
                print_dots(depth, mover.grid.get_size());
                let outcome = self.minimizing_side(mover.grid, alpha, beta, depth + 1);
                best_outcome = max(best_outcome, outcome);
                if best_outcome >= beta {
                    break;
                }
                alpha = max(alpha, best_outcome);
            }
        }

        best_outcome
    }

    fn minimizing_side(
        &self,
        grid: &mut Grid,
        alpha: Outcome,
        mut beta: Outcome,
        depth: usize,
    ) -> Outcome {
        if let Some(outcome) = self.check_finished(grid) {
            return outcome;
        }

        let mut best_outcome = Outcome::Win; // worst outcome

        for ind in 0..grid.get_size() {
            let next_move = Move {
                what: self.min_side,
                ind,
            };
            if let Some(mover) = RevertingMoveMaker::from_move(grid, next_move) {
                print_dots(depth, mover.grid.get_size());
                let outcome = self.maximizing_side(mover.grid, alpha, beta, depth + 1);
                best_outcome = min(best_outcome, outcome);
                if best_outcome <= alpha {
                    break;
                }
                beta = min(beta, best_outcome);
            }
        }

        best_outcome
    }

    fn check_finished(&self, grid: &Grid) -> Option<Outcome> {
        if let Some(winner) = get_winner(grid) {
            if winner == self.max_side {
                Some(Outcome::Win)
            } else {
                Some(Outcome::Loss)
            }
        } else if grid.is_full() {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
}

fn print_dots(count: usize, line_width: usize) {
    print!(
        "\r{:<width$}",
        String::from_utf8(vec![b'.'; count]).unwrap(),
        width = line_width
    );
    io::stdout().flush().unwrap();
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Move {
    what: Mark,
    ind: usize,
}

pub trait MoveMaker<'a>: Sized {
    fn from_move(grid: &'a mut Grid, m: Move) -> Option<Self>;
    fn get_move(&self) -> Move;
}

/// reverting move maker!
struct RevertingMoveMaker<'a> {
    grid: &'a mut Grid,
    saved_move: Move,
}

impl<'a> MoveMaker<'a> for RevertingMoveMaker<'a> {
    fn from_move(grid: &mut Grid, m: Move) -> Option<RevertingMoveMaker> {
        match grid.set_at_ind(m.ind, m.what) {
            Ok(_) => Some(RevertingMoveMaker {
                grid,
                saved_move: m,
            }),
            Err(_) => None,
        }
    }

    fn get_move(&self) -> Move {
        self.saved_move
    }
}

impl Drop for RevertingMoveMaker<'_> {
    fn drop(&mut self) {
        self.grid.unset_at_ind(self.saved_move.ind);
    }
}

/// persistent move maker!
pub struct PersistentMoveMaker {
    saved_move: Move,
}

impl MoveMaker<'_> for PersistentMoveMaker {
    fn from_move(grid: &mut Grid, m: Move) -> Option<PersistentMoveMaker> {
        match grid.set_at_ind(m.ind, m.what) {
            Ok(_) => Some(PersistentMoveMaker { saved_move: m }),
            Err(_) => None,
        }
    }

    fn get_move(&self) -> Move {
        self.saved_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverting_move_maker() {
        let mut g = Grid::new(3);
        {
            let opt_mm = RevertingMoveMaker::from_move(
                &mut g,
                Move {
                    what: Mark::Nought,
                    ind: 4,
                },
            );

            let mm = opt_mm.unwrap();
            assert_eq!(Some(Mark::Nought), mm.grid.get_at_pos(1, 1));
        }
        assert!(g.get_at_pos(1, 1).is_none());
    }

    #[test]
    fn test_persistent_move_maker() {
        let mut g = Grid::new(3);
        {
            let opt_mm = PersistentMoveMaker::from_move(
                &mut g,
                Move {
                    what: Mark::Nought,
                    ind: 4,
                },
            );

            assert!(opt_mm.is_some());
            assert_eq!(Some(Mark::Nought), g.get_at_pos(1, 1));
        }
        assert_eq!(Some(Mark::Nought), g.get_at_pos(1, 1));
    }

    #[test]
    fn test_ai_wins_as_cross() {
        let mut g = Grid::new(3);

        g.set_at_pos(2, 0, Mark::Cross).unwrap();
        g.set_at_pos(2, 2, Mark::Nought).unwrap();
        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(1, 2, Mark::Nought).unwrap();

        let engine = AlphaBetaPruning::new(Mark::Cross);
        engine.try_make_move(&mut g);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                match (x, y) {
                    (2, 0) | (1, 1) | (0, 2) => {
                        assert_eq!(Some(Mark::Cross), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    (2, 2) | (1, 2) => {
                        assert_eq!(Some(Mark::Nought), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    _ => assert!(g.get_at_pos(x, y).is_none(), "x={} y={}", x, y),
                }
            }
        }
    }

    #[test]
    fn test_ai_wins_as_nought() {
        let mut g = Grid::new(3);

        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(0, 0, Mark::Nought).unwrap();
        g.set_at_pos(2, 0, Mark::Cross).unwrap();
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        g.set_at_pos(2, 1, Mark::Cross).unwrap();

        let engine = AlphaBetaPruning::new(Mark::Nought);
        engine.try_make_move(&mut g);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                match (x, y) {
                    (2, 0) | (1, 1) | (2, 1) => {
                        assert_eq!(Some(Mark::Cross), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    (0, 0) | (0, 2) | (0, 1) => {
                        assert_eq!(Some(Mark::Nought), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    _ => assert!(g.get_at_pos(x, y).is_none(), "x={} y={}", x, y),
                }
            }
        }
    }

    #[test]
    fn test_ai_averts_defeat_as_cross() {
        let mut g = Grid::new(3);

        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        g.set_at_pos(0, 0, Mark::Cross).unwrap();
        g.set_at_pos(2, 2, Mark::Nought).unwrap();

        let engine = AlphaBetaPruning::new(Mark::Cross);
        engine.try_make_move(&mut g);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                match (x, y) {
                    (0, 0) | (1, 1) | (1, 2) => {
                        assert_eq!(Some(Mark::Cross), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    (0, 2) | (2, 2) => {
                        assert_eq!(Some(Mark::Nought), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    _ => assert!(g.get_at_pos(x, y).is_none(), "x={} y={}", x, y),
                }
            }
        }
    }

    #[test]
    fn test_ai_averts_defeat_as_nought() {
        let mut g = Grid::new(3);

        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(0, 0, Mark::Nought).unwrap();
        g.set_at_pos(2, 0, Mark::Cross).unwrap();

        let engine = AlphaBetaPruning::new(Mark::Nought);
        engine.try_make_move(&mut g);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                match (x, y) {
                    (2, 0) | (1, 1) => {
                        assert_eq!(Some(Mark::Cross), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    (0, 0) | (0, 2) => {
                        assert_eq!(Some(Mark::Nought), g.get_at_pos(x, y), "x={} y={}", x, y)
                    }
                    _ => assert!(g.get_at_pos(x, y).is_none(), "x={} y={}", x, y),
                }
            }
        }
    }

    #[test]
    fn test_ai_tolerates_full_grid() {
        let mut g = Grid::new(3);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                g.set_at_pos(x, y, Mark::Cross).unwrap();
            }
        }

        let engine = AlphaBetaPruning::new(Mark::Nought);
        engine.try_make_move(&mut g);

        for x in 0..g.get_side_length() {
            for y in 0..g.get_side_length() {
                assert_eq!(Some(Mark::Cross), g.get_at_pos(x, y));
            }
        }
    }
}
