// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    // Simply define the types of the enum
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
