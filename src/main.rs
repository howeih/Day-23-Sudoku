#[macro_use]
extern crate ndarray;
use ndarray::Array2;
use std::collections::HashSet;

struct Sudoku {
    numbers: HashSet<i32>,
    row_dim: usize,
    col_dim: usize,
    matrix: Array2<i32>,
}

impl Sudoku {
    fn new(matrix: Array2<i32>) -> Sudoku {
        let dim = matrix.dim();
        Sudoku {
            numbers: (1..10).collect(),
            row_dim: dim.0,
            col_dim: dim.1,
            matrix: matrix,
        }
    }

    fn find_next(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let next = None;
        let start = self.row_dim * row + col;
        for (d, v) in self.matrix.indexed_iter().skip(start) {
            if *v == 0 {
                return Some(d);
            }
        }
        next
    }

    fn get_numset(&self, row: usize, col: usize) -> HashSet<i32> {
        let mut numset = self.numbers.clone();
        for r in 0..self.row_dim {
            let val = self.matrix[[r, col]];
            if val != 0 {
                numset.remove(&val);
            }
        }
        for c in 0..self.col_dim {
            let val = self.matrix[[row, c]];
            if val != 0 {
                numset.remove(&val);
            }
        }
        numset
    }

    fn get_next_num(&self, num_set: &mut HashSet<i32>) -> i32 {
        let num = *num_set.iter().next().unwrap();
        num_set.remove(&num);
        num
    }

    fn sudoku(&mut self, row: usize, col: usize) -> bool {
        let this_step = self.find_next(row, col);
        if this_step == None {
            return true;
        }
        let this_step = this_step.unwrap();
        let (this_row, this_col) = (this_step.0, this_step.1);
        let mut num_set = self.get_numset(this_row, this_col);
        if num_set.is_empty() {
            self.matrix[[this_row, this_col]] = 0;
            return false;
        }
        let num = self.get_next_num(&mut num_set);
        self.matrix[[this_row, this_col]] = num;
        while !self.sudoku(this_row, this_col) {
            if num_set.is_empty() {
                self.matrix[[this_row, this_col]] = 0;
                return false;
            }
            let num = self.get_next_num(&mut num_set);
            self.matrix[[this_row, this_col]] = num;
        }
        true
    }

    fn get_result(&self) -> &Array2<i32>{
        return &self.matrix
    }
}

fn main() {
    let matrix = array![
        [8, 0, 0, 1, 0, 9, 0, 7, 0],
        [0, 9, 0, 0, 0, 0, 8, 0, 0],
        [5, 0, 3, 0, 4, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 7, 9, 0],
        [0, 0, 7, 2, 6, 5, 3, 0, 0],
        [0, 3, 8, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 9, 0, 4, 0, 1],
        [0, 0, 6, 0, 0, 0, 0, 2, 0],
        [0, 5, 0, 4, 0, 2, 0, 0, 3]
    ];

    let mut s = Sudoku::new(matrix);
    s.sudoku(0, 0);
    println!("{:?}", s.get_result());
}
