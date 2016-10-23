use std::io::{stdin};
use itertools::Itertools;

use board::Board;



pub trait IsolationPlayer {
    fn take_move(&self, board:&mut Board, legal_moves:&Vec<(u16,u16)>) -> (u16, u16);
}

pub struct HumanPlayer;

impl IsolationPlayer for HumanPlayer {

    fn take_move(&self, board:&mut Board, legal_moves:&Vec<(u16,u16)>) -> (u16, u16) {

        let choices = legal_moves
            .iter()
            .enumerate()
            .map(|(i, &(a, b))| format!("{}: ({}, {})", i, a, b))
            .join("\n");

        println!("Make a move: \n{}", choices);

        loop {
            let mut s = String::new();
            stdin().read_line(&mut s).expect("Error getting user input");

            match s.trim().parse::<usize>() {
                Result::Ok(i) => {
                    return legal_moves[i];
                }
                Result::Err(_) => println!("Invalid move")
            }
        }

    }

}
