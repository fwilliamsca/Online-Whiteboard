use std::collections::HashMap;

struct Stroke {
    points: Vec<(i32, i32)>,
    color: String,
}

struct Board {
    strokes: Vec<Stroke>,
    users: Vec<String>,
    chat: Vec<String>,
}

impl Board {
    fn new() -> Self {
        Board {
            strokes: Vec::new(),
            users: Vec::new(),
            chat: Vec::new(),
        }
    }
    fn add_user(&mut self, name: &str) {
        if !self.users.contains(&name.to_string()) {
            self.users.push(name.to_string());
        }
    }
    fn add_stroke(&mut self, pts: Vec<(i32, i32)>, color: &str) {
        self.strokes.push(Stroke {
            points: pts,
            color: color.to_string(),
        });
    }
    fn send_chat(&mut self, msg: &str) {
        self.chat.push(msg.to_string());
    }
    fn summary(&self) {
        println!("Users: {:?}", self.users);
        println!("Strokes: {}", self.strokes.len());
        println!("Chat messages: {}", self.chat.len());
    }
}

fn main() {
    let mut board = Board::new();
    board.add_user("Zarokin");
    board.add_user("Alice");
    board.add_stroke(vec![(10,10),(20,20)], "red");
    board.add_stroke(vec![(15,15),(22,22)], "blue");
    board.send_chat("Hello from Zarokin!");
    board.send_chat("Welcome!");
    board.summary();
}