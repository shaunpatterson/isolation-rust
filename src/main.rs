extern crate itertools;

use itertools::Itertools;

pub mod board;
pub mod player;

use board::Board;
use player::IsolationPlayer;
use player::HumanPlayer;

fn main() {

    let mut board: Board = Board::new(5, 5);

    let mut player1: HumanPlayer = HumanPlayer;

    let legal_moves = vec!((0, 0), (1, 1), (2, 2));

    println!("{}", board.get_legal_moves(0)
        .iter()
        .map(|x| format!("{}", x))
        .join(","));

    board.set_cell(0);
    board.set_cell(1);
    board.set_cell(5);

    println!("{}", board.get_legal_moves(0)
        .iter()
        .map(|x| format!("{}", x))
        .join(","));



    // player1.take_move(&mut board, &legal_moves);

    //println!("{}", strings);
    //println!("Moves: {}", |)

    //println!("Board: \n{}", board);

    //board.apply_move(2);

    //println!("Board: \n{}", board);

}

fn play_isolation(board: &mut Board) {

}
