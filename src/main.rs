use search::search;

pub mod board;
pub mod eval;
pub mod search;

fn main() {
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture).unwrap();

    let mut b = board::Board::default();
    print!("\x1b[?1049h\x1b[2J\x1b[H{b}");

    if std::env::args().count() == 2 {
        input(&mut b);
    }

    loop {
        let (l, eval) = search(b, 9);

        b.play_unchecked(l.m[0]);
        println!("\x1b[2J\x1b[H{b}Eval: {eval}, PV: {l:?}, {:?} side to move", b.side_to_move());

        if b.side_won().is_some() || b.is_full() {
            println!("{:?}", b.side_won());
            break;
        }

        input(&mut b);

        if b.side_won().is_some() || b.is_full() {
            println!("{:?}", b.side_won());
            break;
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture).unwrap();
    std::thread::sleep_ms(5000);
    println!("\x1b[?1049l");
}

fn input(b: &mut board::Board) {
    use crossterm::event::*;

    loop {
        crossterm::terminal::enable_raw_mode().unwrap();
        match read() {
            Ok(Event::Mouse(MouseEvent { kind: MouseEventKind::Down(MouseButton::Left), column, row, .. })) => {
                if column >= 6 && row >= 3 { continue; }
                if b.play(board::Square::new(column as u8 / 2, row as u8)).is_err() { continue; }

                crossterm::terminal::disable_raw_mode().unwrap();
                return;
            },
            Ok(Event::Key(KeyEvent { code: KeyCode::Char('c'), kind: KeyEventKind::Press, modifiers: KeyModifiers::CONTROL, .. })) => {
                crossterm::terminal::disable_raw_mode().unwrap();
                crossterm::execute!(std::io::stdout(), DisableMouseCapture).unwrap();
                println!("\x1b[?1049l");
                std::process::exit(0);
            },
            _ => {},
        }
    }
}
