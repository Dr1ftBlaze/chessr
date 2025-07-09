// src/board/bitboard.rs

/// Bitboard representation for chess engine
/// A BitBoard is a 64-bit integer where each bit corresponds to a square on the chessboard.
/// Bit 0 corresponds to A1, bit 63 corresponds to H8 (little-endian rank-file mapping).

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug,
)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// Create an empty bitboard
    pub const fn empty() -> Self {
        BitBoard(0)
    }

    /// Create a bitboard with a single bit set at the given square index (0..63)
    pub const fn from_square(square: usize) -> Self {
        BitBoard(1u64 << square)
    }

    /// Set a bit at a given square
    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1u64 << square;
    }

    /// Clear a bit at a given square
    pub fn clear_bit(&mut self, square: usize) {
        self.0 &= !(1u64 << square);
    }

    /// Check if a bit at a given square is set
    pub fn is_set(&self, square: usize) -> bool {
        (self.0 & (1u64 << square)) != 0
    }

    /// Count the number of set bits (pieces)
    pub fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    /// Iterator over set bits (squares occupied)
    pub fn iter(&self) -> BitBoardIterator {
        BitBoardIterator(self.0)
    }
}

/// Iterator over set bits in a BitBoard
pub struct BitBoardIterator(u64);

impl Iterator for BitBoardIterator {
    type Item = usize; // square index

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let lsb = self.0.trailing_zeros() as usize;
            self.0 &= self.0 - 1; // clear least significant bit
            Some(lsb)
        }
    }
}

/// Constants for sides
pub mod sides {
    pub const WHITE: usize = 0;
    pub const BLACK: usize = 1;
}

/// Constants for piece types
pub mod pieces {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

/// A full chess position represented by bitboards
#[derive(Clone, Debug)]
pub struct Position {
    /// Bitboards for each side's pieces: [side][piece_type]
    pub bb_pieces: [[BitBoard; 6]; 2],

    /// Bitboards for all pieces of each side
    pub bb_sides: [BitBoard; 2],
}

impl Position {
    /// Create a new empty position
    pub fn new() -> Self {
        Position {
            bb_pieces: [[BitBoard::empty(); 6]; 2],
            bb_sides: [BitBoard::empty(); 2],
        }
    }

    /// Update bb_sides from bb_pieces
    pub fn update_sides(&mut self) {
        self.bb_sides[sides::WHITE] = BitBoard(0);
        self.bb_sides[sides::BLACK] = BitBoard(0);
        for piece in 0..6 {
            self.bb_sides[sides::WHITE].0 |= self.bb_pieces[sides::WHITE][piece].0;
            self.bb_sides[sides::BLACK].0 |= self.bb_pieces[sides::BLACK][piece].0;
        }
    }
}

