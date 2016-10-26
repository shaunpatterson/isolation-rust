use std::fmt;
use std::collections::HashMap;

use types::{ MoveIndex, MoveOffset, MoveTuple };

// http://stackoverflow.com/questions/29981378/is-there-an-easy-way-to-cast-entire-tuples-of-scalar-values-at-once
macro_rules! tuple_as {
    ($t: expr, ($($ty: ident),*)) => {
        {
            let ($($ty,)*) = $t;
            ($($ty as $ty,)*)
        }
    }
}


#[allow(dead_code, unused_variables)]
pub struct Board {
    width: u16,
    height: u16,
    size: u16,
    state: u64,
    move_count: u16
}

impl Board {

    pub fn new(width: u16, height: u16) -> Board {
        Board {
            width: width,
            height: height,
            size: width * height,
            state: 0,
            move_count: 0
        }
    }

    pub fn draw_board(&self, p1: Option<MoveOffset>, p2: Option<MoveOffset>) -> String {

        let p1_last_move = match p1 {
            Some(n) => n as i32,
            _ => -1
        };
        let p2_last_move = match p2 {
            Some(n) => n as i32,
            _ => -1
        };

        let mut output = String::new();
        let b = self.as_2d();

        let mut offset:i32 = 0;
        for (i, row) in b.iter().enumerate() {

            for (j, cell) in row.iter().enumerate() {

                if *cell == 0 {
                    output.push_str(" ");
                } else if offset == p1_last_move {
                    output.push_str("1");
                } else if offset == p2_last_move {
                    output.push_str("2");
                } else {
                    output.push_str("-");
                }

                output.push_str(" | ");

                offset += 1;
            }
            output.push_str("\n");
        }
        output
    }

    pub fn apply_move(&mut self, move_offset: MoveOffset) {
        self.set_cell(move_offset);
        self.move_count += 1;
        println!("State: {}", self.state);
    }

    fn get_cell_by_rc(&self, r: MoveIndex, c: MoveIndex) -> u16 {
        return self.get_cell(r * self.width + c);
    }

    fn get_cell(&self, move_offset:MoveOffset) -> u16 {
        let move_offset = move_offset as u64;
        return (self.state >> move_offset & 1u64) as u16;
    }

    fn set_cell(&mut self, move_offset:MoveOffset) {
        self.state |= 1 << move_offset as u64;
    }

    // Converts a move index (0-size) to (row, col)
    pub fn move_offset_to_move(&self, move_offset:MoveOffset) -> MoveTuple {
        let row = (move_offset / self.width) as u16;
        let col = move_offset % self.width;
        (row, col)
    }

    pub fn get_legal_moves(&self, last_move:Option<MoveOffset>) -> Vec<(MoveOffset, MoveTuple)> {

        let moves = match self.move_count {
            0 | 1 => self.get_first_moves(),
            _ => self.calculate_moves(last_move.expect("Last move is null"))
        };

        moves.iter()
            .map(|x| (*x, self.move_offset_to_move(*x)))
            .collect::<Vec<(MoveOffset, MoveTuple)>>()
    }

    fn is_move_legal(&self, r:i16, c:i16) -> bool {
        return 0 <= r && r < self.height as i16 &&
            0 <= c && c < self.width as i16 &&
            self.get_cell_by_rc(r as u16, c as u16) == 0;
    }

    fn calculate_moves(&self, last_move:MoveOffset) -> Vec<MoveOffset> {
        let (r, c) = self.move_offset_to_move(last_move);
        let r = r as i16;
        let c = c as i16;
        let directions: [(i16, i16); 8] = [ (-1, -1), (-1, 0), (-1, 1),
                                            (0, -1),           (0,  1),
                                            (1, -1), (1,  0),  (1,  1) ];

        let mut fringe = directions.iter()
            .filter(|&&(dr, dc)| self.is_move_legal(r+dr, c+dc))
            .map(|&(dr, dc)| ((r+dr, c+dc), (dr, dc)))
            .collect::<Vec<((i16, i16), (i16, i16))>>();

        let mut moves = vec!();
        while fringe.len() > 0 {
            let next = fringe.pop();

            match next {
                Some(((r, c), (dr, dc))) => {
                    if self.is_move_legal(r, c) {
                        fringe.push(((r+dr, c+dc), (dr, dc)));
                        moves.push((r * self.width as i16 + c) as u16);
                    }
                }
                _ => continue
            }
        }

        moves
    }

    fn get_first_moves(&self) -> Vec<u16> {
        (0..(self.width * self.height))
            .filter(|x| self.get_cell(*x) == 0)
            .collect::<Vec<u16>>()
    }


    //
    // Converts ```board.state``` to a 2d vector
    //
    fn as_2d(&self) -> Vec<Vec<u16>> {
        let mut output = vec![];

        let mut offset = 0;
        for r in 0..self.height {
            let mut row = vec![];

            for c in 0..self.width {
                row.push(self.get_cell(offset));
                offset += 1;
            }

            output.push(row);
        }

        output
    }

}
