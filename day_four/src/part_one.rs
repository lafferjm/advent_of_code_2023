use crate::game;
use crate::input;

pub fn solve() {
    let lines = input::read_input();
    let mut cards: Vec<game::Game> = Vec::new();

    for line in lines {
        let card = game::Game::build(line);
        cards.push(card);
    }

    let answer: i32 = cards
        .iter()
        .map(|x| x.get_winning_count())
        .filter(|x| *x > 0)
        .map(|x| { let base: i32 = 2; return base.pow(x as u32- 1)})
        .sum();

    // let answer1: Vec<usize> = cards
    // .iter()
    // .map(|x| x.get_winning_count())
    // .filter(|x| *x > 0).collect();

    // let answer: Vec<usize> = cards
    //         .iter()
    //         .map(|x| x.get_winning_count())
    //         .filter(|x| *x > 0)
    //         .map(|x| 2 << x).collect();

    // println!("{:?}", answer1);
    println!("{:?}", answer);
}
