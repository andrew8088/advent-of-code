use std::fs;
use std::env;

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win
}

fn to_play(input: &str) -> Play {
    match input {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("got {}, expected A, B, C, X, Y, Z", input),
    }
}

fn get_my_play(their_play: Play, input: &str) -> Play {
    let outcome = match input {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("got {}, expected X, Y, Z", input),
    };

    match (their_play, outcome) {
        (_, Outcome::Draw) => their_play.clone(),

        (Play::Rock, Outcome::Lose) => Play::Scissors,
        (Play::Rock, Outcome::Win) => Play::Paper,
        (Play::Paper, Outcome::Lose) => Play::Rock,
        (Play::Paper, Outcome::Win) => Play::Scissors,
        (Play::Scissors, Outcome::Lose) => Play::Paper,
        (Play::Scissors, Outcome::Win) => Play::Rock,
    }

}

fn score(game: &(Play, Play)) -> usize {
    let mut score = match game.1 {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    };

    score = score + match game {
        (Play::Rock, Play::Rock) => 3,
        (Play::Paper, Play::Paper) => 3,
        (Play::Scissors, Play::Scissors) => 3,

        (Play::Rock, Play::Paper) => 6,
        (Play::Paper, Play::Scissors) => 6,
        (Play::Scissors, Play::Rock) => 6,

        _ => 0,
    };

    score
}

fn score_all(games: Vec<(Play, Play)>) -> usize {
    games.iter().map(|game| score(game)).sum::<usize>()
}

fn main() {
    get_input(|content| {
        let raw_games = content.trim().split("\n").map(|row| row.trim().split(' ').take(2).collect::<Vec<&str>>());

        let games1: Vec<(Play, Play)> = raw_games.to_owned().map(|plays| {
            (to_play(plays[0]), to_play(plays[1]))
        }).collect();

        println!("part 1 score: {}", score_all(games1));

        let games2: Vec<(Play, Play)> = raw_games.map(|plays| {
            let their_play = to_play(plays[0]);
            let my_play = get_my_play(their_play, plays[1]);

            (their_play, my_play)
        }).collect();

        println!("part 2 score: {}", score_all(games2));
    });
}

// TODO make lib for these helpers

fn get_file_name() -> &'static str  {
    if env::var("DEMO").unwrap_or("0".to_string()) == "1" {
        "demo-input.txt"
    } else {
        "input.txt"
    }
}

pub fn get_input<F>(func: F) where F: Fn(String) {
    let file_name = get_file_name();
    println!("loading {}", file_name);
    match fs::read_to_string(file_name) {
        Err(_) => println!("Cannot load file"),
        Ok(content) => {
            func(content);
        }
    }
}
