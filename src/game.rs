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

        let active_player = &self.player1;

        loop {
            println!("Playing isolation");
            println!("{}", self.board);

            //let legal_moves = vec!((0, 0), (1, 1), (2, 2));
            let legal_moves = self.board.get_legal_moves(active_player.last_move());
            active_player.choose_move(&self.board, &legal_moves);
            //active_player.choose_move(&self.board, &legal_moves);

        }

    }

}


