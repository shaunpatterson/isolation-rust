use std::fmt;
use std::collections::HashMap;

use types::{ MoveIndex, MoveOffset, MoveTuple };


#[derive(PartialEq, Eq, Hash)]
pub enum Player {
    O,
    X
}

pub enum Cell {
    BLANK,
    O,
    X
}


#[allow(dead_code, unused_variables)]
pub struct Board {
    width: u16,
    height: u16,
    state: u64,
    move_count: u16,
    turn: Player,
    last_move: HashMap<Player, i16>
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let p1_last_move = self.last_move.get(&Player::O).unwrap();
        let p2_last_move = self.last_move.get(&Player::X).unwrap();

        let b = self.as_2d();

        let mut offset:i16 = 0;
        for (i, row) in b.iter().enumerate() {

            for (j, cell) in row.iter().enumerate() {

                if *cell == 0 {
                    write!(f, "*");
                } else if offset == *p1_last_move {
                    write!(f, "O");
                } else if offset == *p2_last_move {
                    write!(f, "X");
                } else {
                    write!(f, "-");
                }

                write!(f, " | ");

                offset += 1;
            }
            writeln!(f, "");
        }
        Ok(())
    }
}

impl Board {

    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width: width,
            height: height,
            state: 0,
            move_count: 0,
            turn: Player::O,
            last_move: {
                let mut m = HashMap::new();
                m.insert(Player::O, -1);
                m.insert(Player::X, -1);
                m
            }
        }
    }

    pub fn apply_move(&mut self, move_offset: u16) {
        let bitmove = 1 << move_offset;
        self.state |= bitmove;
        self.move_count += 1;

        self.next_player();
    }

    fn next_player(&mut self) {
        match self.turn {
            Player::O => self.turn = Player::X,
            Player::X => self.turn = Player::O
        }
    }

    fn get_cell(&self, move_index:u16) -> u16 {
        return (self.state & (1 << move_index as u64)) as u16;
    }

    pub fn set_cell(&mut self, move_index:u16) {
        self.state |= 1 << move_index as u64;
    }

    // Converts a move index (0-size) to (row, col)
    pub fn move_index_to_move(&self, move_index:u16) -> MoveTuple {
        let row = (move_index / self.width) as u16;
        let col = move_index % self.width;
        (row, col)
    }

    pub fn get_legal_moves(&self, last_move:u16) -> Vec<(MoveOffset, MoveTuple)> {

        let moves = match self.move_count {
            0 | 1 => self.get_first_moves(),
            _ => self.calculate_moves(last_move)
        };

        moves.iter()
            .map(|x| (*x, self.move_index_to_move(*x)))
            .collect::<Vec<(MoveOffset, MoveTuple)>>()
    }

    fn calculate_moves(&self, last_move:u16) -> Vec<MoveOffset> {
        vec!()
    }

    fn get_first_moves(&self) -> Vec<u16> {
        (0..(self.width * self.height))
            .filter(|x| self.get_cell(*x) == 0)
            .collect::<Vec<u16>>()
    }



    //
    // Converts ```board.state``` to a 2d vector
    //
    fn as_2d(&self) -> Vec<Vec<u8>> {
        let mut output = vec![];

        for _ in 1..self.height {
            let mut row = vec![];

            for _ in 1..self.width {
                row.push(1u8);
            }

            output.push(row);
        }

        output
    }

}
