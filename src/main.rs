pub mod board;
pub mod eval;

fn main() {
    let b = board::Board::default();

    for s in b.iter_moves() {
        let mut b = b.clone();
        b.play(s).unwrap();

        println!("{b} {}", eval::eval(b));

        for s in b.iter_moves() {
            let mut b = b.clone();
            b.play(s).unwrap();

            println!("{b} {}", eval::eval(b));
        }
    }
}
