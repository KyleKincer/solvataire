use rayon::prelude::*;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Display, EnumIter)]
pub enum Board {
    Triangle(TriangleBoard),
    English(EnglishBoard),
}

impl Board {
    fn display(&self) {
        match self {
            Board::English(_) => unimplemented!(),
            Board::Triangle(board) => board.display(),
        }
    }

    fn find_valid_moves(&self) -> Vec<Move> {
        match self {
            Board::English(_) => unimplemented!(),
            Board::Triangle(board) => {
                let mut moves: Vec<Move> = Vec::new();
                for (i, row) in board.grid.iter().enumerate() {
                    for (j, &peg) in row.iter().enumerate() {
                        if peg == PegState::Occupied {
                            let odd_row = i % 2 == 1;

                            // Left
                            if j > 1
                                && board.grid[i][j - 1] == PegState::Occupied
                                && board.grid[i][j - 2] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: (i.try_into().unwrap(), (j - 2) as u8),
                                    eliminate: (i.try_into().unwrap(), (j - 1) as u8),
                                });
                            }

                            // Right
                            if j < row.len() - 2
                                && board.grid[i][j + 1] == PegState::Occupied
                                && board.grid[i][j + 2] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: (i.try_into().unwrap(), (j + 2) as u8),
                                    eliminate: (i.try_into().unwrap(), (j + 1) as u8),
                                });
                            }

                            // Left Up
                            if i > 1
                                && j > 0
                                && board.grid[i - 1][j - if odd_row { 0 } else { 1 }]
                                    == PegState::Occupied
                                && board.grid[i - 2][j - 1] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: ((i - 2) as u8, (j - 1) as u8),
                                    eliminate: (
                                        (i - 1) as u8,
                                        (j - if odd_row { 0 } else { 1 }) as u8,
                                    ),
                                });
                            }

                            // Right Up
                            if i > 1
                                && j < row.len() - 1
                                && board.grid[i - 1][j + if odd_row { 1 } else { 0 }]
                                    == PegState::Occupied
                                && board.grid[i - 2][j + 1] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: ((i - 2) as u8, (j + 1) as u8),
                                    eliminate: (
                                        (i - 1) as u8,
                                        (j + if odd_row { 1 } else { 0 }) as u8,
                                    ),
                                });
                            }

                            // Left Down
                            if i < board.grid.len() - 2
                                && j > 0
                                && board.grid[i + 1][j - if odd_row { 0 } else { 1 }]
                                    == PegState::Occupied
                                && board.grid[i + 2][j - 1] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: ((i + 2) as u8, (j - 1) as u8),
                                    eliminate: (
                                        (i + 1) as u8,
                                        (j - if odd_row { 0 } else { 1 }) as u8,
                                    ),
                                });
                            }

                            // Right Down
                            if i < board.grid.len() - 2
                                && j < row.len() - 1
                                && board.grid[i + 1][j + if odd_row { 1 } else { 0 }]
                                    == PegState::Occupied
                                && board.grid[i + 2][j + 1] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i.try_into().unwrap(), j.try_into().unwrap()),
                                    to: ((i + 2) as u8, (j + 1) as u8),
                                    eliminate: (
                                        (i + 1) as u8,
                                        (j + if odd_row { 1 } else { 0 }) as u8,
                                    ),
                                });
                            }
                        }
                    }
                }
                moves
            }
        }
    }

    fn execute_move(&mut self, mov: &Move) {
        match self {
            Board::English(_) => unimplemented!(),
            Board::Triangle(board) => {
                board.grid[mov.from.0 as usize][mov.from.1 as usize] = PegState::Empty;
                board.grid[mov.to.0 as usize][mov.to.1 as usize] = PegState::Occupied;
                board.grid[mov.eliminate.0 as usize][mov.eliminate.1 as usize] = PegState::Empty;
            }
        }
    }

    pub fn solve(&mut self) -> HashSet<Solution> {
        let solutions = Arc::new(Mutex::new(HashSet::new()));

        self.solve_recursive(&solutions, &mut Vec::new());

        let solutions = Arc::try_unwrap(solutions).expect("Lock still has multiple owners");
        solutions.into_inner().expect("Mutex cannot be locked")
    }

    fn solve_recursive(
        &self,
        solutions: &Arc<Mutex<HashSet<Solution>>>,
        move_history: &mut Vec<Move>,
    ) {
        if self.is_solved() {
            // self.display();
            println!("Solved!");
            let mut solutions_guard = solutions.lock().unwrap(); // Acquire the lock
            solutions_guard.insert(Solution::new(move_history.clone())); // Modify the HashSet
            return;
        }
        let moves = self.find_valid_moves();
        if moves.is_empty() {
            self.display();
            // println!("No more moves");
            // clear the terminal
            // print!("{}[2J", 27 as char);
            return;
        }
        moves.par_iter().for_each(|mov| {
            let mut new_board = self.clone();
            new_board.execute_move(mov);

            let mut new_move_history = move_history.clone();
            new_move_history.push(mov.clone());

            let new_solutions = solutions.clone();
            new_board.solve_recursive(&new_solutions, &mut new_move_history);
        });
    }

    fn is_solved(&self) -> bool {
        match self {
            Board::English(_) => unimplemented!(),
            Board::Triangle(board) => {
                let mut count = 0;
                for row in &board.grid {
                    for peg in row {
                        if *peg == PegState::Occupied {
                            count += 1;
                        }
                    }
                }
                count == 1
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Solution {
    moves: Vec<Move>,
}

impl Solution {
    fn new(moves: Vec<Move>) -> Self {
        Solution { moves }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum PegState {
    Occupied,
    Empty,
    Invalid,
}

pub enum InitialHole {
    Center,
    At(usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct TriangleBoard {
    grid: Vec<Vec<PegState>>,
}

impl TriangleBoard {
    pub fn new(size: usize, initial_hole: InitialHole) -> Self {
        let mut grid = vec![vec![PegState::Invalid; size]; size];
        for (i, row) in grid.iter_mut().enumerate() {
            let start = if i == 0 {
                size.div_floor(2) - i
            } else {
                size.div_floor(2) - i.div_ceil(2)
            };
            for peg in &mut row[start..start + i + 1] {
                *peg = PegState::Occupied;
            }
        }
        let center = size.div_floor(2);
        match initial_hole {
            InitialHole::Center => {
                grid[center][center] = PegState::Empty;
            }
            InitialHole::At(i, j) => {
                grid[i][j] = PegState::Empty;
            }
        }

        TriangleBoard { grid }
    }

    fn display(&self) {
        for (i, row) in self.grid.iter().enumerate() {
            if i % 2 == 1 {
                print!(" ");
            }
            for peg in row {
                match peg {
                    PegState::Occupied => print!(" o"),
                    PegState::Empty => print!(" ·"),
                    PegState::Invalid => print!("  "),
                }
            }
            println!();
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
struct EnglishBoard {
    grid: Vec<Vec<PegState>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Move {
    from: (u8, u8),
    to: (u8, u8),
    eliminate: (u8, u8),
}
