#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mark {
    Cross,
    Nought,
}

#[derive(Default, Debug)]
pub struct Field {
    data: Vec<Vec<Option<Mark>>>,
}

impl Field {
    pub fn new(side_length: usize) -> Field {
        Field {
            data: vec![vec![None; side_length]; side_length],
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_at(&self, x: usize, y: usize) -> Option<Mark> {
        self.data[x][y]
    }

    pub fn set_at(&mut self, x: usize, y: usize, what: Mark) -> Result<(), Mark> {
        let sqr = &mut self.data[x][y];

        match *sqr {
            Some(side) => Err(side),
            None => {
                *sqr = Some(what);
                Ok(())
            }
        }
    }

    pub fn unset_at(&mut self, x: usize, y: usize) {
        self.data[x][y] = None;
    }

    pub fn clear(&mut self) {
        for row in &mut self.data {
            row.fill(None);
        }
    }

    pub fn is_full(&self) -> bool {
        for col in &self.data {
            for square in col {
                if square.is_none() {
                    return false;
                }
            }
        }
        true
    }
}

pub fn get_winner(f: &Field) -> Option<Mark> {
    for col_ind in 0..f.get_size() {
        if let Some(w) = get_col_winner(f, col_ind) {
            return Some(w);
        }
    }
    for row_ind in 0..f.get_size() {
        if let Some(w) = get_row_winner(f, row_ind) {
            return Some(w);
        }
    }
    if let Some(w) = get_inc_diag_winner(f) {
        return Some(w);
    }
    if let Some(w) = get_mix_diag_winner(f) {
        return Some(w);
    }

    None
}

fn get_col_winner(f: &Field, col_ind: usize) -> Option<Mark> {
    let col = &f.data[col_ind];
    let first = col[0];
    if col.iter().skip(1).all(|e| *e == first) {
        return first;
    }
    None
}

fn get_row_winner(f: &Field, row_ind: usize) -> Option<Mark> {
    let first = f.data[0][row_ind];
    for col_ind in 1..f.get_size() {
        if f.data[col_ind][row_ind] != first {
            return None;
        }
    }
    first
}

fn get_inc_diag_winner(f: &Field) -> Option<Mark> {
    let first = f.data[0][0];
    for i in 1..f.get_size() {
        if f.data[i][i] != first {
            return None;
        }
    }
    first
}

fn get_mix_diag_winner(f: &Field) -> Option<Mark> {
    let last_ind = f.get_size() - 1;
    let first = f.data[0][last_ind];
    for i in 1..f.get_size() {
        if f.data[i][last_ind - i] != first {
            return None;
        }
    }
    first
}

#[test]
fn test_constructed_field_is_empty() {
    let f = Field::new(5);
    assert_eq!(5, f.get_size());
    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            assert_eq!(None, f.get_at(i, j));
        }
    }
}

#[test]
fn test_set_cross() {
    let mut f = Field::new(3);
    f.set_at(1, 1, Mark::Cross).unwrap();
    assert_eq!(Mark::Cross, f.set_at(1, 1, Mark::Cross).unwrap_err());
    assert_eq!(Mark::Cross, f.set_at(1, 1, Mark::Nought).unwrap_err());

    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            match (i, j) {
                (1, 1) => assert_eq!(Some(Mark::Cross), f.get_at(1, 1)),
                _ => assert_eq!(None, f.get_at(i, j)),
            }
        }
    }

    f.unset_at(1, 1);
    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            assert_eq!(None, f.get_at(i, j));
        }
    }
}

#[test]
fn test_set_nought() {
    let mut f = Field::new(3);
    f.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(Mark::Nought, f.set_at(0, 2, Mark::Nought).unwrap_err());
    assert_eq!(Mark::Nought, f.set_at(0, 2, Mark::Cross).unwrap_err());

    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            match (i, j) {
                (0, 2) => assert_eq!(Some(Mark::Nought), f.get_at(0, 2)),
                _ => assert_eq!(None, f.get_at(i, j)),
            }
        }
    }

    f.unset_at(0, 2);
    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            assert_eq!(None, f.get_at(i, j));
        }
    }
}

#[test]
fn test_clear_field() {
    let mut f = Field::new(3);
    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    f.clear();

    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            assert_eq!(None, f.get_at(i, j));
        }
    }

    f.set_at(1, 1, Mark::Nought).unwrap();
    f.set_at(0, 2, Mark::Cross).unwrap();
}

#[test]
fn test_no_winner() {
    let mut f = Field::new(3);
    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(None, get_winner(&f));
}

#[test]
fn test_col_winner() {
    let mut f = Field::new(3);
    f.set_at(0, 0, Mark::Nought).unwrap();
    f.set_at(0, 1, Mark::Nought).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&f));

    f.clear();
    f.set_at(2, 0, Mark::Cross).unwrap();
    f.set_at(2, 1, Mark::Cross).unwrap();
    f.set_at(2, 2, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&f));
}

#[test]
fn test_row_winner() {
    let mut f = Field::new(3);
    f.set_at(2, 2, Mark::Nought).unwrap();
    f.set_at(0, 2, Mark::Nought).unwrap();
    f.set_at(1, 2, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&f));

    f.clear();
    f.set_at(0, 0, Mark::Cross).unwrap();
    f.set_at(1, 0, Mark::Cross).unwrap();
    f.set_at(2, 0, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&f));
}

#[test]
fn test_diag_winner() {
    let mut f = Field::new(3);
    f.set_at(0, 0, Mark::Cross).unwrap();
    f.set_at(1, 1, Mark::Cross).unwrap();
    f.set_at(2, 2, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&f));

    f.clear();
    f.set_at(0, 2, Mark::Nought).unwrap();
    f.set_at(1, 1, Mark::Nought).unwrap();
    f.set_at(2, 0, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&f));
}

#[test]
fn test_is_full() {
    let mut f = Field::new(3);
    assert!(!f.is_full());

    f.set_at(0, 0, Mark::Cross).unwrap();
    assert!(!f.is_full());

    for i in 0..f.get_size() {
        for j in 0..f.get_size() {
            f.set_at(
                i,
                j,
                if i + j % 2 == 0 {
                    Mark::Cross
                } else {
                    Mark::Nought
                },
            )
            .unwrap_or_default();
        }
    }
    assert!(f.is_full());
}
