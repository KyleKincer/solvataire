#![feature(int_roundings)]

#[derive(Debug)]
enum Board {
    Triangle(TriangleBoard),
    English(EnglishBoard),
}

impl Board {
    fn find_valid_moves(&self) -> Vec<Move> {
        match self {
            Board::English(board) => unimplemented!(),
            Board::Triangle(board) => board.grid.iter().enumerate().flat_map(|(i, row)| {
                row.iter().enumerate().filter_map(|(j, peg)| match peg {
                    PegState::Occupied => {
                        todo!()
                    }
                    _ => None,
                })
            }),
        }
    }
}

#[derive(Debug)]
struct Game {
    board: Board,
    moves: Vec<Move>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PegState {
    Occupied,
    Empty,
    Invalid,
}

enum InitialHole {
    Center,
    At(usize, usize),
}

#[derive(Debug)]
struct TriangleBoard {
    grid: Vec<Vec<PegState>>,
}

impl TriangleBoard {
    fn new(size: usize, initial_hole: Option<InitialHole>) -> Self {
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
        if let Some(hole) = initial_hole {
            match hole {
                InitialHole::Center => {
                    grid[center][center] = PegState::Empty;
                }
                InitialHole::At(i, j) => {
                    grid[i][j] = PegState::Empty;
                }
            }
        } else {
            grid[center][center] = PegState::Empty;
        }

        TriangleBoard { grid }
    }
}

#[derive(Debug)]
struct EnglishBoard {
    grid: Vec<Vec<PegState>>,
}

#[derive(Debug)]
struct Move {
    from: (usize, usize),
    to: (usize, usize),
}

fn main() {
    let mut game = Game {
        board: Board::Triangle(TriangleBoard::new(5, None)),
        moves: Vec::new(),
    };

    println!("{:?}", game);
}
