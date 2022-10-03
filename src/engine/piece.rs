#[derive(Debug)]
pub enum PieceType {
    Pawn,
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
}

pub enum PieceColor {
    Black,
    White,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color,
        }
    }
}
