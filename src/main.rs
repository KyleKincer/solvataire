#![feature(int_roundings)]
mod solver;
mod ui;
use crate::solver::{Board, InitialHole, TriangleBoard};

fn main() {
    let mut board = Board::Triangle(TriangleBoard::new(5, InitialHole::Center));
    let solutions = board.solve();
    println!("Found {} solutions", solutions.len());
}
