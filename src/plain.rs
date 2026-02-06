// Plain I/O for Scoundrel
//
// Takes an instance of Game and just runs through loop.

use crate::game::Game;
use std::io;


fn read_integer(options: &[String]) -> usize {
    let max_val = options.len();
    for (idx, o) in options.iter().enumerate() {
        println!("[{idx}] - {o}");
    }

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(n) if n < max_val => return n,
            _ => println!("Invalid input, please enter a valid integer!"),
        }
    }
}


pub fn run_game(mut game: Game) {
    let fight_opts = vec![
        String::from("Fight barehanded"),
        String::from("Fight with a weapon"),
    ];
    loop {
        println!(
            "=== Remaining cards: {} | Remaining rooms: {}",
            game.deck_len(),
            game.rooms_remaining()
        );

        loop {
            print!("=== Health: {}", game.health);
            if game.weapon > 0 {
                print!(" | Weapon: {}", game.weapon);
                if game.weapon_bound < 14 && game.weapon_bound > 0 {
                    print!(" (max use: {})", game.weapon_bound);
                }
            }

            println!();

            let inputs = game.turn_inputs();
            let input = read_integer(&inputs);

            if game.maybe_end_turn(input) {
                break;
            } else {
                game.play_card(input, || read_integer(&fight_opts));
            }

            if game.maybe_end_turn(0) {
                break;
            }
        }

        if game.maybe_end_game() {
            break;
        }
    }
    
}
