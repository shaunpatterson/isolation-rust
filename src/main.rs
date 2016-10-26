extern crate itertools;

use itertools::Itertools;

pub mod types;
pub mod board;
pub mod player;
pub mod game;

use board::Board;
use player::IsolationPlayer;
use player::HumanPlayer;
use game::Game;

fn main() {

    let mut board: Board = Board::new(5, 5);

    let mut player1: HumanPlayer = HumanPlayer::new();
    let mut player2: HumanPlayer = HumanPlayer::new();

    let mut game = Game::new(board, player1, player2);

    game.play();

}


