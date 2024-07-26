use crate::board::*;

const WIN_EVAL: i16 = i16::MAX / 2;
const LOSE_EVAL: i16 = i16::MIN / 2;

/// Returns the evaluation in the perspective of [`Board::side_to_move`]
pub fn eval(b: Board) -> i16 {
    if let Some(s) = b.side_won() {
        return if s == b.side_to_move() { WIN_EVAL } else { LOSE_EVAL };
    } else if b.is_full() {
        return 0;
    }

    let side = if b.side_to_move() == Side::X { 1 } else { -1 };
    (eval_side(b, Side::X) - eval_side(b, Side::O)) * side
}

fn eval_side(b: Board, s: Side) -> i16 {
    let sbb = b.get_bitboard(s);
    let osbb = b.get_bitboard(-s);
    let pieces = sbb.count_ones() as i16 * 10;

    // 2 connected pieces bonus
    let connect = WIN_PATTERNS.iter().copied().filter(|p| {
        (*p & sbb).count_ones() == 2 && (*p & osbb) == 0
    }).count() as i16 * 50;

    pieces + connect
}
