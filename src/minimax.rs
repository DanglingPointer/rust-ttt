use crate::field::{get_winner, Field, Mark};
use std::cmp::{max, min};

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

    pub fn try_make_move(&self, field: &mut Field) {
        if get_winner(field).is_some() {
            return;
        }

        let mut alpha = Outcome::Loss;
        let beta = Outcome::Win;

        let mut best_outcome = Outcome::Loss; // worst outcome
        let mut best_move: Option<Move> = None;
        let mut last_move: Option<Move> = None;

        for next_move in FieldIterator::new(field.get_size(), self.max_side) {
            if let Some(mover) = RevertingMoveMaker::from_move(field, next_move) {
                last_move = Some(mover.get_move());
                let outcome = self.minimizing_side(mover.field, alpha, beta);
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

        if let Some(bm) = best_move {
            PersistentMoveMaker::from_move(field, bm);
        } else if let Some(lm) = last_move {
            PersistentMoveMaker::from_move(field, lm);
        }
    }

    fn maximizing_side(&self, field: &mut Field, mut alpha: Outcome, beta: Outcome) -> Outcome {
        if let Some(outcome) = self.check_finished(field) {
            return outcome;
        }

        let mut best_outcome = Outcome::Loss; // worst outcome

        for next_move in FieldIterator::new(field.get_size(), self.max_side) {
            if let Some(mover) = RevertingMoveMaker::from_move(field, next_move) {
                let outcome = self.minimizing_side(mover.field, alpha, beta);
                best_outcome = max(best_outcome, outcome);
                if best_outcome >= beta {
                    break;
                }
                alpha = max(alpha, best_outcome);
            }
        }

        best_outcome
    }

    fn minimizing_side(&self, field: &mut Field, alpha: Outcome, mut beta: Outcome) -> Outcome {
        if let Some(outcome) = self.check_finished(field) {
            return outcome;
        }

        let mut best_outcome = Outcome::Win; // worst outcome

        for next_move in FieldIterator::new(field.get_size(), self.min_side) {
            if let Some(mover) = RevertingMoveMaker::from_move(field, next_move) {
                let outcome = self.maximizing_side(mover.field, alpha, beta);
                best_outcome = min(best_outcome, outcome);
                if best_outcome <= alpha {
                    break;
                }
                beta = min(beta, best_outcome);
            }
        }

        best_outcome
    }

    fn check_finished(&self, field: &Field) -> Option<Outcome> {
        if let Some(winner) = get_winner(field) {
            if winner == self.max_side {
                Some(Outcome::Win)
            } else {
                Some(Outcome::Loss)
            }
        } else if field.is_full() {
            Some(Outcome::Draw)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Move {
    pub what: Mark,
    pub x: usize,
    pub y: usize,
}

trait MoveMaker<'a>: Sized {
    fn from_move(field: &'a mut Field, m: Move) -> Option<Self>;
    fn get_move(&self) -> Move;
}

/// field iterator!
struct FieldIterator {
    side: Mark,
    field_size: usize,
    end_pos: usize,
    next_pos: usize,
}

impl FieldIterator {
    fn new(field_size: usize, side: Mark) -> FieldIterator {
        FieldIterator {
            side,
            field_size,
            end_pos: field_size * field_size,
            next_pos: 0,
        }
    }
}

impl Iterator for FieldIterator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos >= self.end_pos {
            return None;
        }
        let current_pos = self.next_pos;
        self.next_pos += 1;

        Some(Move {
            what: self.side,
            x: current_pos / self.field_size,
            y: current_pos % self.field_size,
        })
    }
}

/// reverting move maker!
struct RevertingMoveMaker<'a> {
    field: &'a mut Field,
    saved_move: Move,
}

impl<'a> MoveMaker<'a> for RevertingMoveMaker<'a> {
    fn from_move(field: &mut Field, m: Move) -> Option<RevertingMoveMaker> {
        match field.set_at(m.x, m.y, m.what) {
            Ok(_) => Some(RevertingMoveMaker {
                field,
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
        self.field.unset_at(self.saved_move.x, self.saved_move.y);
    }
}

/// persistent move maker!
struct PersistentMoveMaker {
    saved_move: Move,
}

impl MoveMaker<'_> for PersistentMoveMaker {
    fn from_move(field: &mut Field, m: Move) -> Option<PersistentMoveMaker> {
        match field.set_at(m.x, m.y, m.what) {
            Ok(_) => Some(PersistentMoveMaker { saved_move: m }),
            Err(_) => None,
        }
    }

    fn get_move(&self) -> Move {
        self.saved_move
    }
}

#[test]
fn test_field_iterator() {
    let field_size: usize = 2;
    let expected_moves = vec![
        Move {
            what: Mark::Cross,
            x: 0,
            y: 0,
        },
        Move {
            what: Mark::Cross,
            x: 0,
            y: 1,
        },
        Move {
            what: Mark::Cross,
            x: 1,
            y: 0,
        },
        Move {
            what: Mark::Cross,
            x: 1,
            y: 1,
        },
    ];

    let mut obtained_moves = Vec::<Move>::new();

    for next_move in FieldIterator::new(field_size, Mark::Cross) {
        obtained_moves.push(next_move);
    }

    assert_eq!(expected_moves, obtained_moves);
}

#[test]
fn test_reverting_move_maker() {
    let mut f = Field::new(3);
    {
        let opt_mm = RevertingMoveMaker::from_move(
            &mut f,
            Move {
                what: Mark::Nought,
                x: 1,
                y: 1,
            },
        );

        let mm = opt_mm.unwrap();
        assert_eq!(Some(Mark::Nought), mm.field.get_at(1, 1));
    }
    assert!(f.get_at(1, 1).is_none());
}

#[test]
fn test_persistent_move_maker() {
    let mut f = Field::new(3);
    {
        let opt_mm = PersistentMoveMaker::from_move(
            &mut f,
            Move {
                what: Mark::Nought,
                x: 1,
                y: 1,
            },
        );

        assert!(opt_mm.is_some());
        assert_eq!(Some(Mark::Nought), f.get_at(1, 1));
    }
    assert_eq!(Some(Mark::Nought), f.get_at(1, 1));
}

#[test]
fn test_ai_wins_as_cross() {
    let mut f = Field::new(3);

    f.set_at(2, 0, Mark::Cross).unwrap();
    f.set_at(2, 2, Mark::Nought).unwrap();
    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(1, 2, Mark::Nought).unwrap();

    let engine = AlphaBetaPruning::new(Mark::Cross);
    engine.try_make_move(&mut f);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            match (x, y) {
                (2, 0) | (1, 1) | (0, 2) => {
                    assert_eq!(Some(Mark::Cross), f.get_at(x, y), "x={} y={}", x, y)
                }
                (2, 2) | (1, 2) => {
                    assert_eq!(Some(Mark::Nought), f.get_at(x, y), "x={} y={}", x, y)
                }
                _ => assert!(f.get_at(x, y).is_none(), "x={} y={}", x, y),
            }
        }
    }
}

#[test]
fn test_ai_wins_as_nought() {
    let mut f = Field::new(3);

    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(0, 0, Mark::Nought).unwrap();
    f.set_at(2, 0, Mark::Cross).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    f.set_at(2, 1, Mark::Cross).unwrap();

    let engine = AlphaBetaPruning::new(Mark::Nought);
    engine.try_make_move(&mut f);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            match (x, y) {
                (2, 0) | (1, 1) | (2, 1) => {
                    assert_eq!(Some(Mark::Cross), f.get_at(x, y), "x={} y={}", x, y)
                }
                (0, 0) | (0, 2) | (0, 1) => {
                    assert_eq!(Some(Mark::Nought), f.get_at(x, y), "x={} y={}", x, y)
                }
                _ => assert!(f.get_at(x, y).is_none(), "x={} y={}", x, y),
            }
        }
    }
}

#[test]
fn test_ai_averts_defeat_as_cross() {
    let mut f = Field::new(3);

    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    f.set_at(0, 0, Mark::Cross).unwrap();
    f.set_at(2, 2, Mark::Nought).unwrap();

    let engine = AlphaBetaPruning::new(Mark::Cross);
    engine.try_make_move(&mut f);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            match (x, y) {
                (0, 0) | (1, 1) | (1, 2) => {
                    assert_eq!(Some(Mark::Cross), f.get_at(x, y), "x={} y={}", x, y)
                }
                (0, 2) | (2, 2) => {
                    assert_eq!(Some(Mark::Nought), f.get_at(x, y), "x={} y={}", x, y)
                }
                _ => assert!(f.get_at(x, y).is_none(), "x={} y={}", x, y),
            }
        }
    }
}

#[test]
fn test_ai_averts_defeat_as_nought() {
    let mut f = Field::new(3);

    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(0, 0, Mark::Nought).unwrap();
    f.set_at(2, 0, Mark::Cross).unwrap();

    let engine = AlphaBetaPruning::new(Mark::Nought);
    engine.try_make_move(&mut f);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            match (x, y) {
                (2, 0) | (1, 1) => assert_eq!(Some(Mark::Cross), f.get_at(x, y), "x={} y={}", x, y),
                (0, 0) | (0, 2) => {
                    assert_eq!(Some(Mark::Nought), f.get_at(x, y), "x={} y={}", x, y)
                }
                _ => assert!(f.get_at(x, y).is_none(), "x={} y={}", x, y),
            }
        }
    }
}

#[test]
fn test_ai_tolerates_full_field() {
    let mut f = Field::new(3);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            f.set_at(x, y, Mark::Cross).unwrap();
        }
    }

    let engine = AlphaBetaPruning::new(Mark::Nought);
    engine.try_make_move(&mut f);

    for x in 0..f.get_size() {
        for y in 0..f.get_size() {
            assert_eq!(Some(Mark::Cross), f.get_at(x, y));
        }
    }
}
