use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
                                    from: (i, j),
                                    to: (i, j - 2),
                                    eliminate: (i, j - 1),
                                });
                            }

                            // Right
                            if j < row.len() - 2
                                && board.grid[i][j + 1] == PegState::Occupied
                                && board.grid[i][j + 2] == PegState::Empty
                            {
                                moves.push(Move {
                                    from: (i, j),
                                    to: (i, j + 2),
                                    eliminate: (i, j + 1),
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
                                    from: (i, j),
                                    to: (i - 2, j - 1),
                                    eliminate: (i - 1, j - if odd_row { 0 } else { 1 }),
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
                                    from: (i, j),
                                    to: (i - 2, j + 1),
                                    eliminate: (i - 1, j + if odd_row { 1 } else { 0 }),
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
                                    from: (i, j),
                                    to: (i + 2, j - 1),
                                    eliminate: (i + 1, j - if odd_row { 0 } else { 1 }),
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
                                    from: (i, j),
                                    to: (i + 2, j + 1),
                                    eliminate: (i + 1, j + if odd_row { 1 } else { 0 }),
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
                board.grid[mov.from.0][mov.from.1] = PegState::Empty;
                board.grid[mov.to.0][mov.to.1] = PegState::Occupied;
                board.grid[mov.eliminate.0][mov.eliminate.1] = PegState::Empty;
            }
        }
    }

    pub fn solve(&mut self) -> HashSet<Solution> {
        let mut move_history = Vec::new();
        let mut solutions = HashSet::new();
        self.solve_recursive(&mut solutions, &mut move_history);
        solutions
    }

    fn solve_recursive(&self, solutions: &mut HashSet<Solution>, move_history: &mut Vec<Move>) {
        if self.is_solved() {
            // self.display();
            // println!("Solved!");
            solutions.insert(Solution::new(self.clone(), move_history.clone()));
            return;
        }
        let moves = self.find_valid_moves();
        if moves.is_empty() {
            // self.display();
            // println!("No more moves");
            // clear the terminal
            // print!("{}[2J", 27 as char);
            return;
        }
        for mov in &moves {
            let mut new_board = self.clone();
            new_board.execute_move(mov);
            // new_board.display();
            move_history.push(mov.clone());
            new_board.solve_recursive(solutions, move_history);
            move_history.pop();
        }
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
    board: Board,
    moves: Vec<Move>,
}

impl Solution {
    fn new(board: Board, moves: Vec<Move>) -> Self {
        Solution { board, moves }
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TriangleBoard {
    grid: Vec<Vec<PegState>>,
}

impl TriangleBoard {
    pub fn new(size: usize, initial_hole: InitialHole) -> Self {
        let mut grid = vec![vec![PegState::Invalid; size]; size];
        for i in 0..size {
            let mut start = 0;
            if i == 0 {
                start = size.div_floor(2) - i;
            } else {
                start = size.div_floor(2) - i.div_ceil(2);
            }
            for j in start..start + i + 1 {
                grid[i][j] = PegState::Occupied;
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct EnglishBoard {
    grid: Vec<Vec<PegState>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
    eliminate: (usize, usize),
}
