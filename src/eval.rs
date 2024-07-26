use crate::board::*;

const WIN_EVAL: i16 = i16::MAX;
const LOSE_EVAL: i16 = i16::MIN;

/// Returns the evaluation in the perspective of [`Board::side_to_move`]
pub fn eval(b: Board) -> i16 {
    if let Some(s) = b.side_won() {
        return if s == b.side_to_move() { WIN_EVAL } else { LOSE_EVAL };
    }

    (eval_side(b, Side::X) - eval_side(b, Side::O)) * if b.side_to_move() == Side::X { 1 } else { -1 }
}

fn eval_side(b: Board, s: Side) -> i16 {
    b.get_bitboard(s).count_ones() as i16 * 100
}
