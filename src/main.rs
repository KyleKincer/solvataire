#![feature(int_roundings)]
#![feature(let_chains)]
mod solver;
mod ui;
use crate::solver::{Board, InitialHole, TriangleBoard};
use crate::ui::start;

fn main() {
    // let mut board = Board::Triangle(TriangleBoard::new(5, InitialHole::Center));
    // let solutions = board.solve();
    // println!("Found {} solutions", solutions.len());
    start();
}
