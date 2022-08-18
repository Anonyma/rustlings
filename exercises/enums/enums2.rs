// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // ADDED
    Quit,
    Move {x: i32, y: i32},
    Echo(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }

    // ADDED
    fn Echo(&self, message: String) {
        println!("{:?}", &self);
    }
    
    fn ChangeColor(&self, x: i32, y: i32, z: i32) {

    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
