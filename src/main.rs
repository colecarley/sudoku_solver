use std::time::Instant;
//import sudoku

pub struct SudokuSolver {}

impl SudokuSolver {
    fn new() -> Self {
        SudokuSolver {}
    }

    pub fn check_row(&self, grid: &[[u8; 9]; 9], row: u8, possible: u8) -> bool {
        let mut good: bool = true;
        for i in 0..9 {
            if grid[row as usize][i] == possible {
                good = false;
            }
        }
        return good;
    }

    pub fn check_column(&self, grid: &[[u8; 9]; 9], col: u8, possible: u8) -> bool {
        let mut good: bool = true;
        for i in 0..9 {
            if grid[i][col as usize] == possible {
                good = false;
            }
        }
        return good;
    }

    pub fn check_box(&self, grid: &[[u8; 9]; 9], row: u8, col: u8, possible: u8) -> bool {
        let mut good: bool = true;
        let row = row - (row % 3);
        let col = col - (col % 3);
        for i in 0..3 {
            for j in 0..3 {
                if grid[(row + i) as usize][(col + j) as usize] == possible {
                    good = false;
                }
            }
        }
        return good;
    }

    pub fn check(&self, grid: &[[u8; 9]; 9], row: u8, col: u8, possible: u8) -> bool {
        return self.check_row(grid, row, possible)
            && self.check_column(grid, col, possible)
            && self.check_box(grid, row, col, possible);
    }

    pub fn get_next(&self, grid: &mut [[u8; 9]; 9], (row, col): (u8, u8)) -> (u8, u8) {
        let next: (u8, u8);
        if col == 8 {
            next = (row + 1, 0);
        } else {
            next = (row, col + 1);
        }

        if next.0 > 8 {
        } else if grid[next.0 as usize][next.1 as usize] != 0 {
            return self.get_next(grid, next);
        }

        // println!("last: {:?}, next: {:?}", (row, col), next);
        return next;
    }
}

pub fn solve(solver: &SudokuSolver, grid: &mut [[u8; 9]; 9], row: u8, col: u8, found: &mut bool) {
    if row > 8 {
        *found = true;
        return;
    }

    for i in 1..10 {
        if solver.check(grid, row, col, i) {
            grid[row as usize][col as usize] = i;

            let (next_row, next_col) = solver.get_next(grid, (row, col));
            solve(solver, grid, next_row, next_col, found);

            if *found {
                break;
            }
            grid[row as usize][col as usize] = 0;
        }
    }
}

fn main() {
    let mut grid: [[u8; 9]; 9] = [
        [0, 0, 0, 3, 0, 0, 0, 6, 0],
        [0, 6, 4, 0, 0, 1, 0, 2, 0],
        [0, 0, 0, 5, 0, 0, 0, 0, 0],
        [0, 2, 3, 0, 5, 0, 6, 0, 0],
        [0, 0, 9, 8, 0, 3, 7, 0, 0],
        [0, 0, 8, 0, 9, 0, 1, 4, 0],
        [0, 0, 0, 0, 0, 9, 0, 0, 0],
        [0, 1, 0, 4, 0, 0, 8, 9, 0],
        [0, 5, 0, 0, 0, 7, 0, 0, 0],
    ];

    let solver = SudokuSolver::new();

    let mut found = false;
    let start = solver.get_next(&mut grid, (0, 0));

    let start_time = Instant::now();
    solve(&solver, &mut grid, start.0, start.1, &mut found);
    let duration = start_time.elapsed();

    println!("{:?}", grid);
    println!("Solving took {:?}ms", duration);
}
