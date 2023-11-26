use std::io;
use rand::Rng;

fn main() {
    println!("==============================================");
    println!("Welcome to Rust Paper Scissors!");
    println!("Simply type: r, p, s");
    println!("or q to quit");
    println!("==============================================");

    let moves: [&str; 3] = ["r", "p", "s"];

    loop {
        //creates a new mutable variable equal to a string
        let mut player_move: String = String::new();
        /*
        create a new variable with the name random_number and an inter type of unsigned size
        that is equal to the thread_rng().gen_range method from the rand cargo
        */
        let random_number: usize = rand::thread_rng().gen_range(0..3);
        let computer_move: &str = moves[random_number];

        println!("");
        println!("Type your move!");

        io::stdin()
            // here I reference the string by borrowing it and say I would like to mutate it
            .read_line(&mut player_move)
            .expect("Error Reading Line");

        //trim method return string slice
        let trim_player_move: String = player_move.trim().to_lowercase();
        let trim_player_move_ref: &str = trim_player_move.as_str();

        println!("Computer chose: {}", computer_move);

        if trim_player_move_ref == computer_move {
            println!("Tie Game!");
            continue;
        }

        match trim_player_move_ref {
            "r" => {
                if computer_move == "s" {
                    println!("Player has won");
                } else {
                    println!("Computer has won");
                }
            }
            "p" => {
                if computer_move == "r" {
                    println!("Player has won");
                } else {
                    println!("Computer has won");
                }
            }
            "s" => {
                if computer_move == "r" {
                    println!("Player has won");
                } else {
                    println!("Computer has won");
                }
            }
            "q" => {
                break;
            }

            &_ => {
                println!("Invalid!");
            }
        }
    }
}
