use std::io::{stdin};
use itertools::Itertools;

use types::*;
use board::Board;


pub trait IsolationPlayer {
    fn choose_move(&self, board: &Board, last_move:&Option<MoveOffset>) -> MoveOffset;
    fn take_move(&mut self, move_index: u16);
    fn last_move(&self) -> Option<u16>;
}

pub struct HumanPlayer {
    last_move: Option<u16>
}

pub struct ComputerPlayer {
    last_move: Option<u16>
}

impl HumanPlayer {

    pub fn new() -> HumanPlayer {
        HumanPlayer {
            last_move: None
        }
    }

}

impl IsolationPlayer for HumanPlayer {

    fn choose_move(&self, board: &Board, last_move:&Option<MoveOffset>) -> MoveOffset {
        let mut legal_moves = board.get_legal_moves(last_move);
        legal_moves.sort_by_key(|k| k.0);

        let choices = legal_moves
            .iter()
            .enumerate()
            .map(|(i, &(o, (a, b)))| format!("{}: {} - ({}, {})", i, o, a, b))
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
        self.last_move = Some(move_index);
    }

    fn last_move(&self) -> Option<u16> {
        self.last_move
    }

}

impl ComputerPlayer {
    pub fn new() -> ComputerPlayer {
        ComputerPlayer {
            last_move: None
        }
    }

    fn utility(&self, board: &Board) -> i64 {
        0i64
    }

    fn terminal_state(&self, board: &Board, last_move:&Option<MoveOffset>, depth: usize, max_depth: usize) -> bool {
        depth >= max_depth || board.get_legal_moves(last_move).len() == 0
    }

    fn min_value(&self, board: &mut Board, last_move:&Option<MoveOffset>, depth: usize, max_depth: usize) -> i64 {
        0i64
    }

    fn max_value(&self, board: &mut Board, last_move:&Option<MoveOffset>, depth: usize, max_depth: usize) -> i64 {
        if self.terminal_state(board, last_move, depth, max_depth) {
            return 1i64;
        }
        1i64
    }

    fn minmax(&self, board: &mut Board, last_move:&Option<MoveOffset>, max_depth:usize, maximizing_player: bool) {
        println!("Successors: {:?}", board.successors(last_move));
    }
}

impl IsolationPlayer for ComputerPlayer {

    fn choose_move(&self, board: &Board, last_move:&Option<MoveOffset>) -> MoveOffset {
        let mut working_board = *board;

        self.minmax(&mut working_board, last_move, 4, true);

        0u16
    }


    fn take_move(&mut self, move_index: u16) {
        self.last_move = Some(move_index);
    }

    fn last_move(&self) -> Option<u16> {
        self.last_move
    }

}
