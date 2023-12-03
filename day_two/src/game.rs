#[derive(Debug)]
pub struct Game {
    pub red_count: i32,
    pub green_count: i32,
    pub blue_count: i32,
}

impl Game {
    fn build(line: &str) -> Self {
        let parts: Vec<&str> = line.split(',').collect();
        let mut red_count = 0;
        let mut green_count= 0;
        let mut blue_count = 0;

        for part in parts {
            let split: Vec<&str> = part.trim().split(' ').collect();
            let (count, color) = (split[0], split[1]);

            if color == "red" {
                red_count = count.parse().unwrap();
            } 

            if color == "green" {
                green_count = count.parse().unwrap();
            }

            if color == "blue" {
                blue_count = count.parse().unwrap();
            }
        }

        Game{
            red_count,
            green_count,
            blue_count,
        }
    }

    fn is_valid(self: &Self) -> bool {
        self.red_count <= 12 && self.green_count <= 13 && self.blue_count <= 14
    }

    pub fn round_valid(turns: &str) -> bool {
        let mut round_results: Vec<bool> = Vec::new();
        let turns: Vec<&str> = turns.split(';').collect();
        for turn in turns {
            let game = Self::build(turn);
            round_results.push(game.is_valid());
        }

        !round_results.iter().any(|x| *x == false)
    }
}