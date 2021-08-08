use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Mark {
    Cross,
    Nought,
}

#[derive(Default, Debug)]
pub struct Grid {
    side_length: usize,
    data: Vec<Option<Mark>>,
}

impl Grid {
    pub fn new(side_length: usize) -> Grid {
        Grid {
            side_length,
            data: vec![None; side_length * side_length],
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_side_length(&self) -> usize {
        self.side_length
    }

    pub fn get_at_ind(&self, ind: usize) -> Option<Mark> {
        self.data[ind]
    }

    pub fn get_at_pos(&self, x: usize, y: usize) -> Option<Mark> {
        self.get_at_ind(y * self.side_length + x)
    }

    pub fn set_at_ind(&mut self, ind: usize, what: Mark) -> Result<(), Mark> {
        let sqr = &mut self.data[ind];
        match *sqr {
            Some(side) => Err(side),
            None => {
                *sqr = Some(what);
                Ok(())
            }
        }
    }

    pub fn set_at_pos(&mut self, x: usize, y: usize, what: Mark) -> Result<(), Mark> {
        self.set_at_ind(y * self.side_length + x, what)
    }

    pub fn unset_at_ind(&mut self, ind: usize) {
        self.data[ind] = None;
    }

    pub fn is_full(&self) -> bool {
        self.data.iter().all(|e| e.is_some())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "\n    ")?;
        for col in 0..self.side_length {
            write!(formatter, "{:^3} ", col)?;
        }
        for row in 0..self.side_length {
            write!(formatter, "\n{:^3}|", row)?;
            for col in 0..self.side_length {
                let symbol = match self.get_at_pos(col, row) {
                    Some(Mark::Cross) => 'X',
                    Some(Mark::Nought) => 'O',
                    None => ' ',
                };
                write!(formatter, "{:^3}|", symbol)?;
            }
        }
        write!(formatter, "\n")?;
        Ok(())
    }
}

pub fn get_winner(g: &Grid) -> Option<Mark> {
    fn get_col_winner(g: &Grid, col_ind: usize) -> Option<Mark> {
        let first = g.data[col_ind];
        let start_ind = col_ind + g.side_length;
        let slice: &[Option<Mark>] = &g.data[start_ind..];
        if slice.iter().step_by(g.side_length).all(|e| *e == first) {
            first
        } else {
            None
        }
    }
    for col_ind in 0..g.get_side_length() {
        if let Some(w) = get_col_winner(g, col_ind) {
            return Some(w);
        }
    }

    fn get_row_winner(g: &Grid, row_ind: usize) -> Option<Mark> {
        let start_ind = row_ind * g.side_length;
        let end_ind = start_ind + g.side_length;
        let first = g.data[start_ind];
        let slice = &g.data[start_ind + 1..end_ind];
        if slice.iter().all(|e| *e == first) {
            first
        } else {
            None
        }
    }
    for row_ind in 0..g.get_side_length() {
        if let Some(w) = get_row_winner(g, row_ind) {
            return Some(w);
        }
    }

    fn get_inc_diag_winner(g: &Grid) -> Option<Mark> {
        let first = g.data[0];
        let interval = g.side_length + 1;
        let slice: &[Option<Mark>] = &g.data[interval..];
        if slice.iter().step_by(interval).all(|e| *e == first) {
            first
        } else {
            None
        }
    }
    if let Some(w) = get_inc_diag_winner(g) {
        return Some(w);
    }

    fn get_mix_diag_winner(g: &Grid) -> Option<Mark> {
        let interval = g.side_length - 1;
        let start_ind = interval + interval;
        let end_ind = g.data.len() - 1;
        let first = g.data[interval];
        let slice: &[Option<Mark>] = &g.data[start_ind..end_ind];
        if slice.iter().step_by(interval).all(|e| *e == first) {
            first
        } else {
            None
        }
    }
    if let Some(w) = get_mix_diag_winner(g) {
        return Some(w);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructed_grid_is_empty() {
        let g = Grid::new(5);
        assert_eq!(5, g.get_side_length());
        for i in 0..g.get_side_length() {
            for j in 0..g.get_side_length() {
                assert_eq!(None, g.get_at_pos(i, j));
            }
        }
    }

    #[test]
    fn test_set_cross() {
        let mut g = Grid::new(3);
        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        assert_eq!(Mark::Cross, g.set_at_pos(1, 1, Mark::Cross).unwrap_err());
        assert_eq!(Mark::Cross, g.set_at_pos(1, 1, Mark::Nought).unwrap_err());

        for i in 0..g.get_side_length() {
            for j in 0..g.get_side_length() {
                match (i, j) {
                    (1, 1) => assert_eq!(Some(Mark::Cross), g.get_at_pos(1, 1)),
                    _ => assert_eq!(None, g.get_at_pos(i, j)),
                }
            }
        }
    }

    #[test]
    fn test_set_nought() {
        let mut g = Grid::new(3);
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        assert_eq!(Mark::Nought, g.set_at_pos(0, 2, Mark::Nought).unwrap_err());
        assert_eq!(Mark::Nought, g.set_at_pos(0, 2, Mark::Cross).unwrap_err());

        for i in 0..g.get_side_length() {
            for j in 0..g.get_side_length() {
                match (i, j) {
                    (0, 2) => assert_eq!(Some(Mark::Nought), g.get_at_pos(0, 2)),
                    _ => assert_eq!(None, g.get_at_pos(i, j)),
                }
            }
        }
    }

    #[test]
    fn test_no_winner() {
        let mut g = Grid::new(3);
        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        assert_eq!(None, get_winner(&g));
    }

    #[test]
    fn test_col_winner() {
        let mut g = Grid::new(3);
        g.set_at_pos(0, 0, Mark::Nought).unwrap();
        g.set_at_pos(0, 1, Mark::Nought).unwrap();
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        assert_eq!(Some(Mark::Nought), get_winner(&g));

        g = Grid::new(3);
        g.set_at_pos(2, 0, Mark::Cross).unwrap();
        g.set_at_pos(2, 1, Mark::Cross).unwrap();
        g.set_at_pos(2, 2, Mark::Cross).unwrap();
        assert_eq!(Some(Mark::Cross), get_winner(&g));
    }

    #[test]
    fn test_row_winner() {
        let mut g = Grid::new(3);
        g.set_at_pos(2, 2, Mark::Nought).unwrap();
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        g.set_at_pos(1, 2, Mark::Nought).unwrap();
        assert_eq!(Some(Mark::Nought), get_winner(&g));

        g = Grid::new(3);
        g.set_at_pos(0, 0, Mark::Cross).unwrap();
        g.set_at_pos(1, 0, Mark::Cross).unwrap();
        g.set_at_pos(2, 0, Mark::Cross).unwrap();
        assert_eq!(Some(Mark::Cross), get_winner(&g));
    }

    #[test]
    fn test_diag_winner() {
        let mut g = Grid::new(3);
        g.set_at_pos(0, 0, Mark::Cross).unwrap();
        g.set_at_pos(1, 1, Mark::Cross).unwrap();
        g.set_at_pos(2, 2, Mark::Cross).unwrap();
        assert_eq!(Some(Mark::Cross), get_winner(&g));

        g = Grid::new(3);
        g.set_at_pos(0, 2, Mark::Nought).unwrap();
        g.set_at_pos(1, 1, Mark::Nought).unwrap();
        g.set_at_pos(2, 0, Mark::Nought).unwrap();
        assert_eq!(Some(Mark::Nought), get_winner(&g));
    }

    #[test]
    fn test_is_full() {
        let mut g = Grid::new(3);
        assert!(!g.is_full());

        g.set_at_pos(0, 0, Mark::Cross).unwrap();
        assert!(!g.is_full());

        for i in 0..g.get_side_length() {
            for j in 0..g.get_side_length() {
                let _ = g.set_at_pos(
                    i,
                    j,
                    if i + j % 2 == 0 {
                        Mark::Cross
                    } else {
                        Mark::Nought
                    },
                );
            }
        }
        assert!(g.is_full());
    }
}
