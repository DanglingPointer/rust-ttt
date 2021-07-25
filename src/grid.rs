#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mark {
    Cross,
    Nought,
}

#[derive(Default, Debug)]
pub struct Grid {
    data: Vec<Vec<Option<Mark>>>,
}

impl Grid {
    pub fn new(side_length: usize) -> Grid {
        Grid {
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

pub fn get_winner(g: &Grid) -> Option<Mark> {
    for col_ind in 0..g.get_size() {
        if let Some(w) = get_col_winner(g, col_ind) {
            return Some(w);
        }
    }
    for row_ind in 0..g.get_size() {
        if let Some(w) = get_row_winner(g, row_ind) {
            return Some(w);
        }
    }
    if let Some(w) = get_inc_diag_winner(g) {
        return Some(w);
    }
    if let Some(w) = get_mix_diag_winner(g) {
        return Some(w);
    }

    None
}

fn get_col_winner(g: &Grid, col_ind: usize) -> Option<Mark> {
    let col = &g.data[col_ind];
    let first = col[0];
    if col.iter().skip(1).all(|e| *e == first) {
        return first;
    }
    None
}

fn get_row_winner(g: &Grid, row_ind: usize) -> Option<Mark> {
    let first = g.data[0][row_ind];
    for col_ind in 1..g.get_size() {
        if g.data[col_ind][row_ind] != first {
            return None;
        }
    }
    first
}

fn get_inc_diag_winner(g: &Grid) -> Option<Mark> {
    let first = g.data[0][0];
    for i in 1..g.get_size() {
        if g.data[i][i] != first {
            return None;
        }
    }
    first
}

fn get_mix_diag_winner(g: &Grid) -> Option<Mark> {
    let last_ind = g.get_size() - 1;
    let first = g.data[0][last_ind];
    for i in 1..g.get_size() {
        if g.data[i][last_ind - i] != first {
            return None;
        }
    }
    first
}

#[test]
fn test_constructed_grid_is_empty() {
    let g = Grid::new(5);
    assert_eq!(5, g.get_size());
    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            assert_eq!(None, g.get_at(i, j));
        }
    }
}

#[test]
fn test_set_cross() {
    let mut g = Grid::new(3);
    g.set_at(1, 1, Mark::Cross).unwrap();
    assert_eq!(Mark::Cross, g.set_at(1, 1, Mark::Cross).unwrap_err());
    assert_eq!(Mark::Cross, g.set_at(1, 1, Mark::Nought).unwrap_err());

    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            match (i, j) {
                (1, 1) => assert_eq!(Some(Mark::Cross), g.get_at(1, 1)),
                _ => assert_eq!(None, g.get_at(i, j)),
            }
        }
    }

    g.unset_at(1, 1);
    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            assert_eq!(None, g.get_at(i, j));
        }
    }
}

#[test]
fn test_set_nought() {
    let mut g = Grid::new(3);
    g.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(Mark::Nought, g.set_at(0, 2, Mark::Nought).unwrap_err());
    assert_eq!(Mark::Nought, g.set_at(0, 2, Mark::Cross).unwrap_err());

    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            match (i, j) {
                (0, 2) => assert_eq!(Some(Mark::Nought), g.get_at(0, 2)),
                _ => assert_eq!(None, g.get_at(i, j)),
            }
        }
    }

    g.unset_at(0, 2);
    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            assert_eq!(None, g.get_at(i, j));
        }
    }
}

#[test]
fn test_clear_grid() {
    let mut g = Grid::new(3);
    g.set_at(1, 1, Mark::Cross).unwrap();
    g.set_at(0, 2, Mark::Nought).unwrap();
    g.clear();

    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            assert_eq!(None, g.get_at(i, j));
        }
    }

    g.set_at(1, 1, Mark::Nought).unwrap();
    g.set_at(0, 2, Mark::Cross).unwrap();
}

#[test]
fn test_no_winner() {
    let mut g = Grid::new(3);
    g.set_at(1, 1, Mark::Cross).unwrap();
    g.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(None, get_winner(&g));
}

#[test]
fn test_col_winner() {
    let mut g = Grid::new(3);
    g.set_at(0, 0, Mark::Nought).unwrap();
    g.set_at(0, 1, Mark::Nought).unwrap();
    g.set_at(0, 2, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&g));

    g.clear();
    g.set_at(2, 0, Mark::Cross).unwrap();
    g.set_at(2, 1, Mark::Cross).unwrap();
    g.set_at(2, 2, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&g));
}

#[test]
fn test_row_winner() {
    let mut g = Grid::new(3);
    g.set_at(2, 2, Mark::Nought).unwrap();
    g.set_at(0, 2, Mark::Nought).unwrap();
    g.set_at(1, 2, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&g));

    g.clear();
    g.set_at(0, 0, Mark::Cross).unwrap();
    g.set_at(1, 0, Mark::Cross).unwrap();
    g.set_at(2, 0, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&g));
}

#[test]
fn test_diag_winner() {
    let mut g = Grid::new(3);
    g.set_at(0, 0, Mark::Cross).unwrap();
    g.set_at(1, 1, Mark::Cross).unwrap();
    g.set_at(2, 2, Mark::Cross).unwrap();
    assert_eq!(Some(Mark::Cross), get_winner(&g));

    g.clear();
    g.set_at(0, 2, Mark::Nought).unwrap();
    g.set_at(1, 1, Mark::Nought).unwrap();
    g.set_at(2, 0, Mark::Nought).unwrap();
    assert_eq!(Some(Mark::Nought), get_winner(&g));
}

#[test]
fn test_is_full() {
    let mut g = Grid::new(3);
    assert!(!g.is_full());

    g.set_at(0, 0, Mark::Cross).unwrap();
    assert!(!g.is_full());

    for i in 0..g.get_size() {
        for j in 0..g.get_size() {
            g.set_at(
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
    assert!(g.is_full());
}
