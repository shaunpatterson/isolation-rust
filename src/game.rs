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

}


