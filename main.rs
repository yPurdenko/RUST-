extern crate blackjack;
use std::io;
use std::io::{stdin, stdout, Write};

fn init_players() -> Vec<String> {
    println!("please enter count of players: ");
    let mut input_text = String::new();
    let mut players: Vec<String> = vec![];
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut count = 0;
    match trimmed.parse::<u32>() {
        Ok(i) => count = i,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };

    for _i in 0..count {
        println!("enter players name: ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        players.push(s);
    }
    players
}

fn main() {
    let names = init_players();
    let mut game = blackjack::blackjack_game::BlackJackGame::new(names);
    game.play();
}
