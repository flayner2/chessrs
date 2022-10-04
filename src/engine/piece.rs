#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    None,
    King,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White = 8,
    Black = 16,
}

#[derive(Debug)]
pub struct Piece {
    piece_type: PieceType,
    piece_color: PieceColor,
}

impl Piece {
    pub fn new(piece_type: PieceType, piece_color: PieceColor) -> Self {
        Self {
            piece_type,
            piece_color,
        }
    }

    pub fn is_piece_in_square(piece: Piece, square_value: i8) -> bool {
        ((piece.piece_type as i8) | (piece.piece_color as i8)) == square_value
    }

    pub fn get_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}
