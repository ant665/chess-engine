use std::collections::BTreeMap;

pub struct Game{
    turn: Team,
    chess_board: BTreeMap<u8, Vec<Case>>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            turn: Team::White,
            chess_board: BTreeMap::from([
            (1, vec![
                Case{position: Position{x:1, y:1}, piece_on: Some(Piece::Rook(Team::White))}, 
                Case{position: Position{x:2, y:1}, piece_on: Some(Piece::Knight(Team::White))},
                Case{position: Position{x:3, y:1}, piece_on: Some(Piece::Bishop(Team::White))},
                Case{position: Position{x:4, y:1}, piece_on: Some(Piece::Queen(Team::White))},
                Case{position: Position{x:5, y:1}, piece_on: Some(Piece::King(Team::White))},
                Case{position: Position{x:6, y:1}, piece_on: Some(Piece::Bishop(Team::White))},
                Case{position: Position{x:7, y:1}, piece_on: Some(Piece::Knight(Team::White))},
                Case{position: Position{x:8, y:1}, piece_on: Some(Piece::Rook(Team::White))}
            ]),
            (2, vec![
                Case{position: Position{x:1, y:2}, piece_on: Some(Piece::Pawn(Team::White))}, 
                Case{position: Position{x:2, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:3, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:4, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:5, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:6, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:7, y:2}, piece_on: Some(Piece::Pawn(Team::White))},
                Case{position: Position{x:8, y:2}, piece_on: Some(Piece::Pawn(Team::White))}
            ]),
            (3, vec![
                Case{position: Position{x:1, y:3}, piece_on: None}, 
                Case{position: Position{x:2, y:3}, piece_on: None},
                Case{position: Position{x:3, y:3}, piece_on: None},
                Case{position: Position{x:4, y:3}, piece_on: None},
                Case{position: Position{x:5, y:3}, piece_on: None},
                Case{position: Position{x:6, y:3}, piece_on: None},
                Case{position: Position{x:7, y:3}, piece_on: None},
                Case{position: Position{x:8, y:3}, piece_on: None}
            ]),
            (4, vec![
                Case{position: Position{x:1, y:4}, piece_on: None}, 
                Case{position: Position{x:2, y:4}, piece_on: None},
                Case{position: Position{x:3, y:4}, piece_on: None},
                Case{position: Position{x:4, y:4}, piece_on: None},
                Case{position: Position{x:5, y:4}, piece_on: None},
                Case{position: Position{x:6, y:4}, piece_on: None},
                Case{position: Position{x:7, y:4}, piece_on: None},
                Case{position: Position{x:8, y:4}, piece_on: None}
            ]),
            (5, vec![
                Case{position: Position{x:1, y:5}, piece_on: None}, 
                Case{position: Position{x:2, y:5}, piece_on: None},
                Case{position: Position{x:3, y:5}, piece_on: None},
                Case{position: Position{x:4, y:5}, piece_on: None},
                Case{position: Position{x:5, y:5}, piece_on: None},
                Case{position: Position{x:6, y:5}, piece_on: None},
                Case{position: Position{x:7, y:5}, piece_on: None},
                Case{position: Position{x:8, y:5}, piece_on: None}
            ]),
            (6, vec![
                Case{position: Position{x:1, y:6}, piece_on: None}, 
                Case{position: Position{x:2, y:6}, piece_on: None},
                Case{position: Position{x:3, y:6}, piece_on: None},
                Case{position: Position{x:4, y:6}, piece_on: None},
                Case{position: Position{x:5, y:6}, piece_on: None},
                Case{position: Position{x:6, y:6}, piece_on: None},
                Case{position: Position{x:7, y:6}, piece_on: None},
                Case{position: Position{x:8, y:6}, piece_on: None}
            ]),
            (7, vec![
                Case{position: Position{x:1, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:2, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:3, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:4, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:5, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:6, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:7, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
                Case{position: Position{x:8, y:7}, piece_on: Some(Piece::Pawn(Team::Black))}, 
            ]),
            (8, vec![
                Case{position: Position{x:1, y:8}, piece_on: Some(Piece::Rook(Team::Black))}, 
                Case{position: Position{x:2, y:8}, piece_on: Some(Piece::Knight(Team::Black))},
                Case{position: Position{x:3, y:8}, piece_on: Some(Piece::Bishop(Team::Black))},
                Case{position: Position{x:4, y:8}, piece_on: Some(Piece::Queen(Team::Black))},
                Case{position: Position{x:5, y:8}, piece_on: Some(Piece::King(Team::Black))},
                Case{position: Position{x:6, y:8}, piece_on: Some(Piece::Bishop(Team::Black))},
                Case{position: Position{x:7, y:8}, piece_on: Some(Piece::Knight(Team::Black))},
                Case{position: Position{x:8, y:8}, piece_on: Some(Piece::Rook(Team::Black))}
            ]),
        ])}
    }

    pub fn move_piece(self, from_pos: Position, to_pos: Position) -> Result<Case, ChessError>{
        todo!();
    }

    pub fn is_check(self) {
        todo!();
        self.is_checkmate();
    }

    pub fn is_checkmate(self) {
        todo!();
    }

    pub fn translate_position_struct_to_chess_position(self) {
        todo!();
    }
}

enum ChessError {
    MoveError { message: String },
}

struct Position {
    x: u8,
    y: u8,
}

struct Case {
    position: Position,
    piece_on: Option<Piece>,
}

enum Team {
    White,
    Black,
}

enum Piece {
    Pawn(Team),
    Rook(Team),
    Bishop(Team),
    Knight(Team),
    Queen(Team),
    King(Team),
}
