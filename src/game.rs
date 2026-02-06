use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp;

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

#[derive(Debug)]
pub struct Game {
    pub health: i32,
    pub weapon: i32,
    pub weapon_bound: i32,
    score: i32,
    deck: Vec<Card>,
    room: Vec<Card>,
    can_avoid_room: bool,
    last_card: Option<Card>,
}

impl Game {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = Vec::with_capacity(44);
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

        let room: Vec<Card> = cards.drain(..4).collect();

        Self {
            health: 20,
            weapon: 0,
            weapon_bound: 0,
            score: -208, // TODO: calc from cards in deck.
            deck: cards,
            room,
            can_avoid_room: true,
            last_card: None,
        }
    }

    pub fn rooms_remaining(&mut self) -> usize {
        if self.deck.is_empty() {
            0
        } else {
            self.deck.len() / 3 + 1
        }
    }

    pub fn deck_len(&mut self) -> usize {
        self.deck.len()
    }

    pub fn turn_inputs(&mut self) -> Vec<String> {
        let mut inputs: Vec<String> = Vec::new();

        for r in &self.room {
            inputs.push(r.formatted());
        }

        if self.can_avoid_room && self.room.len() == 4 {
            inputs.push(String::from("Avoid this room"));
        }
        inputs
    }

    pub fn maybe_end_turn(&mut self, input: usize) -> bool {
        if input == 4 {
            let mut rng = thread_rng();
            self.can_avoid_room = false;
            self.room.shuffle(&mut rng);
            self.deck.append(&mut self.room);
            self.room = self.deck.drain(..4).collect();
            return true;
        }

        if self.health == 0 || self.room.is_empty() {
            return true;
        }

        if self.room.len() == 1 && !self.deck.is_empty() {
            let new_card_count = cmp::min(3, self.deck.len());
            self.room
                .append(&mut self.deck.drain(..new_card_count).collect());
            self.can_avoid_room = self.deck.len() >= 4;
            return true;
        }
        false
    }

    pub fn maybe_end_game(&mut self) -> bool {
        // Death :(
        if self.health == 0 {
            println!("You died.");
            println!("Your score: {}", self.score);
            return true;
        }

        // Win
        if self.deck.is_empty() {
            println!("You win!");

            self.score = self.health;
            if let Some(c) = self.last_card {
                if c.is_potion() && self.health == 20 {
                    self.score += c.value;
                }
            }
            println!("Your score: {}", self.score);
            return true;
        }
        false
    }

    pub fn play_card<F>(&mut self, idx: usize, mut request_fight_choice: F)
    where
        F: FnMut() -> usize,
    {
        let played = self.room.remove(idx);

        match played.suit {
            Suit::Potion => {
                self.health = cmp::min(20, self.health + played.value);
            }
            Suit::Weapon => {
                self.weapon = played.value;
                self.weapon_bound = 14;
            }
            Suit::Monster => {
                if self.weapon_bound >= played.value && self.weapon > 0 {
                    let fight_opt = request_fight_choice();
                    // Barehanded
                    if fight_opt == 0 {
                        self.health = cmp::max(0, self.health - played.value);
                    // With a weapon
                    } else {
                        self.health =
                            (self.health + self.weapon - played.value).clamp(0, self.health);
                        self.weapon_bound = played.value;
                    }
                } else {
                    // Barehanded
                    self.health = cmp::max(0, self.health - played.value);
                }
                self.score += played.value;
            }
        }
        self.last_card = Some(played);
    }
}
