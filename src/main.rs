#![feature(int_roundings)]

#[derive(Debug)]
enum Board {
    Triangle(TriangleBoard),
    English(EnglishBoard),
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

#[derive(Debug)]
struct TriangleBoard {
    grid: Vec<Vec<PegState>>,
}

impl TriangleBoard {
    fn new(size: usize) -> Self {
        let mut grid = vec![vec![PegState::Invalid; size]];
        for i in 0..size {
            let mut start = 0;
            if i % 2 != 0 || i == 0 {
                start = size.div_floor(2) - i;
            } else {
                start = size.div_floor(2) - (i - 1);
            }
            for j in start..start + i + 1 {
                grid[i][j] = PegState::Occupied;
            }
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
        board: Board::Triangle(TriangleBoard::new(5)),
        moves: Vec::new(),
    };
    println!("{:?}", game);
}
