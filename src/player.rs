use std::io::{stdin};
use itertools::Itertools;

use types::*;
use board::Board;



pub trait IsolationPlayer {
    fn choose_move(&self, board: &Board, legal_moves:&Vec<(MoveOffset, MoveTuple)>) -> MoveOffset;
    fn take_move(&mut self, move_index: u16);
    fn last_move(&self) -> (u16);
}

pub struct HumanPlayer {
    last_move: u16
}

impl HumanPlayer {

    pub fn new() -> HumanPlayer {
        HumanPlayer {
            last_move: 0
        }
    }

}

impl IsolationPlayer for HumanPlayer {

    fn choose_move(&self, board: &Board, legal_moves:&Vec<(MoveOffset, MoveTuple)>) -> MoveOffset {

        let choices = legal_moves
            .iter()
            .enumerate()
            .map(|(i, &(o, (a, b)))| format!("{}: ({}, {})", i, a, b))
            .join("\n");

        println!("Make a move: \n{}", choices);

        loop {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Error getting user input");

            match s.trim().parse::<usize>() {
                Result::Ok(i) => {
                    return legal_moves[i].0;
                }
                Result::Err(_) => println!("Invalid move")
            }
        }

    }

    fn take_move(&mut self, move_index: u16) {
        self.last_move = move_index;
    }

    fn last_move(&self) -> u16 {
        self.last_move
    }

}
