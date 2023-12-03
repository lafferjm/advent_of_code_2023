use crate::input;
use crate::game;

pub fn solve() {
    let lines = input::get_input();
    let mut round_sum = 0;

    for line in lines {
        let round_game_parts: Vec<&str> = line.split(':').collect();
        let (round, turns) = (round_game_parts[0], round_game_parts[1]);

        let is_valid = game::Game::round_valid(turns);

        if is_valid {
            let round = input::parse_round(round);
            round_sum += round;
        }
    }

    println!("{}", round_sum);
}