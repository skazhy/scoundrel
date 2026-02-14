// Plain I/O for Scoundrel
//
// Takes an instance of Game and just runs through loop.

use crate::game::{FightMode, Game};
use std::io;

fn print_inputs(options: &[String]) {
    for (idx, o) in options.iter().enumerate() {
        println!("[{idx}] - {o}");
    }
}

fn read_integer(options: &[String]) -> usize {
    let max_val = options.len();

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

    let room_opts = vec![
        String::from("Avoid this room"),
        String::from("Play this room"),
    ];
    loop {
        // Room loop
        println!(
            "=== Remaining cards: {} | Remaining rooms: {}",
            game.deck_len(),
            game.rooms_remaining()
        );

        loop {
            // Card loop
            print!("=== Health: {}", game.health);
            if game.weapon > 0 {
                print!(" | Weapon: {}", game.weapon);
                if game.weapon_bound < 14 && game.weapon_bound > 0 {
                    print!(" (max use: {})", game.weapon_bound);
                }
            }
            println!();

            let inputs = game.turn_inputs();
            print_inputs(&inputs);

            if game.can_avoid_room() {
                println!("===");
                print_inputs(&room_opts);
                if read_integer(&room_opts) == 0 {
                    game.avoid_room();
                    break;
                } else {
                    game.play_room();
                    print_inputs(&inputs);
                }
            }

            let mut f = FightMode::Barehanded;
            let input = read_integer(&inputs);
            if game.needs_fight_mode(input) {
                print_inputs(&fight_opts);
                if read_integer(&fight_opts) == 1 {
                    f = FightMode::Weapon;
                }
            }

            game.play_card(input, f);
            if game.maybe_end_turn() {
                break;
            }
        }

        if game.maybe_end_game() {
            break;
        }
    }
}
