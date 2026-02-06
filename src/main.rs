use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;
use std::io;

#[derive(Debug, Clone, Copy)]
enum Suit {
    Monster,
    Weapon,
    Potion,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    value: i32,
    suit: Suit,
}

impl Card {
    fn formatted(self) -> String {
        match self.suit {
            Suit::Monster => format!("Monster {}", self.value),
            Suit::Weapon => format!("Weapon {}", self.value),
            Suit::Potion => format!("Potion: {}", self.value),
        }
    }

    fn is_potion(self) -> bool {
        matches!(self.suit, Suit::Potion)
    }
}

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

fn main() {
    let mut cards: Vec<Card> = Vec::with_capacity(44);
    let mut health = 20;
    let mut last_card: Option<Card> = None;
    let mut weapon = 0;
    let mut score: i32 = -208;
    let mut weapon_bound = 14;
    let fight_opts = vec![
        String::from("Fight barehanded"),
        String::from("Fight with a weapon"),
    ];

    for i in 2..=10 {
        cards.push(Card {
            value: i,
            suit: Suit::Monster,
        });
        cards.push(Card {
            value: i,
            suit: Suit::Monster,
        });
        cards.push(Card {
            value: i,
            suit: Suit::Weapon,
        });
        cards.push(Card {
            value: i,
            suit: Suit::Potion,
        });
    }

    for i in 11..=14 {
        cards.push(Card {
            value: i,
            suit: Suit::Monster,
        });
        cards.push(Card {
            value: i,
            suit: Suit::Monster,
        });
    }

    let mut rng = thread_rng();
    cards.shuffle(&mut rng);

    // Initial room
    let mut room_size: usize = 4;
    let mut room: Vec<Card> = cards.drain(..room_size).collect();

    let mut can_avoid_room = cards.len() > 4;

    loop {
        let rooms_remaining = if cards.len() == 0 { 0 } else { cards.len() / 3 + 1 };
        println!("=== Remaining cards: {} | Remaining rooms: {rooms_remaining}", cards.len());

        loop {
            print!("=== Health: {health}");
            if weapon > 0 {
                print!(" | Weapon: {weapon}");
                if weapon_bound < 14 {
                    print!(" (max use: {weapon_bound})");
                }
            }
            println!();

            let mut inputs: Vec<String> = Vec::new();
            if can_avoid_room && room.len() == 4 {
                inputs.push(String::from("Avoid this room"));
            }

            for r in &room {
                inputs.push(r.formatted());
            }
            let mut input = read_integer(&inputs);

            if input == 0 && can_avoid_room && room.len() == 4 {
                break;
            } else {
                if can_avoid_room && room.len() == 4 {
                    input -= 1;
                }

                match room[input].suit {
                    Suit::Potion => {
                        health = cmp::min(20, health + room[input].value);
                    }
                    Suit::Weapon => {
                        weapon = room[input].value;
                        weapon_bound = 14;
                    }
                    Suit::Monster => {
                        if weapon_bound >= room[input].value && weapon > 0 {
                            let fight_opt = read_integer(&fight_opts);
                            // Barehanded
                            if fight_opt == 0 {
                                health = cmp::max(0, health - room[input].value);
                            // With a weapon
                            } else {
                                health = (health + weapon - room[input].value).clamp(0, health);
                                weapon_bound = room[input].value;
                            }
                        } else {
                            // Barehanded
                            health = cmp::max(0, health - room[input].value);
                        }
                        score += room[input].value;
                    }
                }
                last_card = Some(room.remove(input));
            }

            if (room.len() == 1 && !cards.is_empty()) || room.is_empty() || health == 0 {
                break;
            }
        }

        room_size = cmp::min(4, cards.len());

        // Last room avoided
        if room.len() == 4 {
            can_avoid_room = false;
            room.shuffle(&mut rng);
            cards.append(&mut room);
            room = cards.drain(..room_size).collect();
            continue;
        }

        // Death :(
        if health == 0 {
            println!("You died.");
            break;
        }
        // Win
        if cards.is_empty() {
            println!("You win!");

            score = health;
            if let Some(c) = last_card {
                if c.is_potion() && health == 20 {
                    score += c.value;
                }
            }
            break;
        }

        room.append(&mut cards.drain(..cmp::min(3, room_size)).collect());
        can_avoid_room = cards.len() >= 4;
    }
    println!("Your score: {score}");
}
