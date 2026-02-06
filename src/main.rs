mod game;
mod plain;

use game::Game;
use plain::run_game;


fn main() {
    let game = Game::new();
    run_game(game);
}
