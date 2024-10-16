use clap::*;
use smoltttbot::*;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    me: bool,
    #[arg(short, long)]
    recover: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut b = board::Board::default();

    if let Some(r) = &args.recover {
        for c in r.as_bytes().iter().rev() {
            b.x = (b.x << 1) | (*c == b'x') as u16;
            b.o = (b.o << 1) | (*c == b'o') as u16;
        }
    }

    print!("\x1b[?1049h\x1b[2J\x1b[H{b}");
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture).unwrap();

    if args.me { input(&mut b); }

    loop {
        let (l, eval, count) = search::search(b, 9);

        b.play_unchecked(l.m[0]);
        println!("\x1b[2J\x1b[H{b}Eval: {eval}, PV: {l:?}, Evaluated: {count}, {:?} side to move", b.side_to_move());

        if b.side_won().is_some() || b.is_full() {
            println!("{:?}", b.side_won());
            break;
        }

        input(&mut b);

        if b.side_won().is_some() || b.is_full() {
            println!("\x1b[2J\x1b[H{b}");
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
                if column >= 6 || row >= 3 { continue; }
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
