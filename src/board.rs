#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Board {
    // 00 │ 01 │ 02
    // ───┼────┼───
    // 04 │ 05 │ 06
    // ───┼────┼───
    // 08 │ 09 │ 10
    x: u16,
    o: u16,
    side_to_move: Side,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    #[default]
    X,
    O,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Square(u8); // 0000 yyxx

impl Board {
    pub fn side_to_move(&self) -> Side { self.side_to_move }

    pub fn check(&self) {
        assert_eq!(self.x & self.o, 0);
    }

    pub fn side_won(&self) -> Option<Side> {
        if Self::is_win(self.x) {
            Some(Side::X)
        } else if Self::is_win(self.o) {
            Some(Side::O)
        } else {
            None
        }
    }

    fn is_win(b: u16) -> bool {
        WIN_PATTERNS.iter().copied().any(|p| b & p == p)
    }

    pub fn get_bitboard(&self, side: Side) -> u16 {
        match side {
            Side::X => self.x,
            Side::O => self.o,
        }
    }

    fn side_to_move_board(&mut self) -> &mut u16 {
        match self.side_to_move {
            Side::X => &mut self.x,
            Side::O => &mut self.o,
        }
    }

    pub fn play(&mut self, m: Square) -> Result<(), ()> {
        if (self.x | self.o) & m.bitboard() != 0 || self.side_won().is_some() {
            return Err(());
        }

        self.play_unchecked(m);
        Ok(())
    }

    pub fn play_unchecked(&mut self, m: Square) {
        *self.side_to_move_board() |= m.bitboard();
        self.null_move();
    }

    pub fn null_move(&mut self) {
        self.side_to_move = -self.side_to_move;
    }

    pub fn get_side_at(&self, s: Square) -> Option<Side> {
        if self.x & s.bitboard() != 0 {
            Some(Side::X)
        } else if self.o & s.bitboard() != 0 {
            Some(Side::O)
        } else {
            None
        }
    }

    pub fn iter_moves(&self) -> MoveIter {
        MoveIter {
            board: self.x | self.o,
            square_idx: self.side_won().map_or(0, |_| usize::MAX),
        }
    }

    pub fn is_full(&self) -> bool {
        self.x | self.o == 0b111_0111_0111
    }
}

impl core::ops::Neg for Side {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl Square {
    pub const fn new(x: u8, y: u8) -> Self {
        assert!(x < 3);
        assert!(y < 3);
        Self::new_unchecked(x, y)
    }

    pub const fn new_unchecked(x: u8, y: u8) -> Self { Self(x | (y << 2)) }

    pub const fn x(self) -> u8 { self.0 & 3 }
    pub const fn y(self) -> u8 { self.0 >> 2 }

    const fn bitboard(self) -> u16 {
        1 << self.0
    }
}

pub const WIN_PATTERNS: &[u16] = &[
    0b001_0001_0001, // vert
    0b010_0010_0010,
    0b100_0100_0100,
    0b111_0000_0000, // horiz
    0b000_0111_0000,
    0b000_0000_0111,
    0b100_0010_0001, // diag
    0b001_0010_0100,
];

pub const SQUARES: &[Square] = &[
    Square::new(0, 0),
    Square::new(1, 0),
    Square::new(2, 0),
    Square::new(0, 1),
    Square::new(1, 1),
    Square::new(2, 1),
    Square::new(0, 2),
    Square::new(1, 2),
    Square::new(2, 2),
];

use core::fmt;
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..3 {
            for x in 0..3 {
                match self.get_side_at(Square::new_unchecked(x, y)) {
                    Some(Side::X) => write!(f, "\x1b[41mX \x1b[0m"),
                    Some(Side::O) => write!(f, "\x1b[42mO \x1b[0m"),
                    None => write!(f, "- "),
                }?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.x())
            .field(&self.y())
            .finish()
    }
}

pub struct MoveIter {
    board: u16,
    square_idx: usize,
}

impl Iterator for MoveIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        while self.square_idx < SQUARES.len() {
            let sq = SQUARES[self.square_idx];
            self.square_idx += 1;

            if self.board & sq.bitboard() != 0 { continue; }

            return Some(sq);
        }

        None
    }
}
