use board::Board;
use player::IsolationPlayer;

pub struct Game<P: IsolationPlayer> {
    board: Board,
    player1: P,
    player2: P
}

impl<P: IsolationPlayer> Game<P> {

    pub fn new(board: Board, player1: P, player2: P) -> Game<P> {
        Game {
            board: board,
            player1: player1,
            player2: player2
        }
    }

    pub fn play(&mut self) {

        let mut active_player_index = false;

        loop {
            println!("Playing isolation");
            println!("Active player {}", (active_player_index as u16) + 1);
            println!("{}", self.board.draw_board(self.player1.last_move(), self.player2.last_move()));

            let mut active_player = match active_player_index {
                false => &mut self.player1,
                true => &mut self.player2
            };

            let legal_moves = self.board.get_legal_moves(active_player.last_move());
            if legal_moves.len() == 0 {
                match active_player_index {
                    false => println!("Player 1 lost"),
                    true => println!("Player 2 lost")
                };
                println!("{} won", active_player_index);
                break;
            }


            let chosen_move = active_player.choose_move(&self.board, &legal_moves);

            println!("Chosen move offset: {}", chosen_move);

            active_player.take_move(chosen_move);
            self.board.apply_move(chosen_move);

            println!("Chosen move: {:?}", chosen_move);

            active_player_index = !active_player_index;
        }

    }

}


