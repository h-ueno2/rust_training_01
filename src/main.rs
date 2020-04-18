use std::sync::Arc;
use std::cell::Cell;

fn main() {
    let x: Message = Message::Move { x: 3, y: 4 };
    let y: BoardGameTrun = BoardGameTrun::Move { squares: 1 };
}

enum Message {
    Quit,
    ChangeColor(i32, i32, i32).
    Move { x: i32, y: i32 },
    Write(String),
}

enum BoardGameTrun {
    Move { squares: i32 }.
    Pass,
}
