use crate::input;
use crate::game;

pub fn solve() {
    let lines = input::get_input();
    let mut powers_sum: i64 = 0;

    for line in lines {
        let round_game_parts: Vec<&str> = line.split(':').collect();
        let turns = round_game_parts[1];

        let round_power = game::Game::round_powers(turns);
        powers_sum += round_power;
    }

    println!("{}", powers_sum);
}