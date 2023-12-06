pub struct Game {
    pub winning_numbers: Vec<i32>,
    pub game_numbers: Vec<i32>,
}

impl Game {
    fn get_numbers(numbers: &str) -> Vec<i32> {
        numbers
            .trim()
            .split(' ')
            .filter(|x| x.len() > 0)
            .map(|x| -> i32 { x.parse().unwrap() })
            .collect()
    }

    pub fn build(line: String) -> Self {
        let line_parts: Vec<&str> = line.split(":").collect();
        let line_parts: Vec<&str> = line_parts[1].split("|").collect();

        let winning_numbers = Self::get_numbers(line_parts[0]);
        let game_numbers = Self::get_numbers(line_parts[1]);

        Game {
            winning_numbers,
            game_numbers,
        }
    }

    pub fn get_winning_count(&self) -> usize {
        self.game_numbers.iter().filter(|&x| self.winning_numbers.contains(x)).collect::<Vec<&i32>>().len()
    }
}
