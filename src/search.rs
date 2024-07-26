use crate::{board::*, eval::*};

pub fn search(b: Board, depth: usize) -> (Line, i16) {
    let mut line = Line::default();

    let eval = -search_inner(b, i16::MIN / 2 - 69, i16::MAX / 2 + 69, depth, &mut line);

    (line, eval)
}

fn search_inner(b: Board, mut alpha: i16, beta: i16, depth: usize, pline: &mut Line) -> i16 {
    if depth == 0 || b.is_full() || b.side_won().is_some() {
        pline.c = 0;
        return eval(b);
    }

    let mut line = Line::default();

    for m in b.iter_moves() {
        let mut b = b.clone();
        b.play(m).unwrap();

        let eval = -search_inner(b, -beta, -alpha, depth - 1, &mut line);

        if eval >= beta { return beta; }

        if eval > alpha {
            alpha = eval;

            pline.m[0] = m;
            pline.m[1..line.c + 1].copy_from_slice(&line.m[..line.c]);
            pline.c = line.c + 1;
        }
    }

    alpha
}

#[derive(Default)]
pub struct Line {
    pub m: [Square; 9],
    pub c: usize,
}

impl core::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.m[..self.c].fmt(f)
    }
}
