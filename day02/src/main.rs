
// 1:02 start (31.1.2023)
// 1:34 part 1 done

use input_downloader::*;

#[derive(PartialEq,Debug,Clone,Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors
}
use RPS::*;

impl RPS {
    fn score(&self) -> i32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3
        }
    }

    fn score_for_round(me: RPS, opponent: RPS) -> i32 {
        let outcome_score = Outcome::of(me, opponent).score();
        let shape_score = me.score();
        outcome_score + shape_score
    }

    fn parse(c: char) -> RPS {
        match c {
            'A' | 'X' => Rock,
            'B' | 'Y' => Paper,
            'C' | 'Z' => Scissors,
            _ => panic!("invalid char")
        }
    }
}

#[derive(PartialEq,Debug)]
enum Outcome {
    Lose,
    Draw,
    Win
}
use Outcome::*;

impl Outcome {
    fn of(me: RPS, opponent: RPS) -> Outcome {
        if me == opponent {
            return Draw;
        }
        match (me, opponent) {
            (Rock, Paper) => Lose,
            (Rock, Scissors) => Win,
            (Paper, Rock) => Win,
            (Paper, Scissors) => Lose,
            (Scissors, Rock) => Lose,
            (Scissors, Paper)=> Win,
            _ => panic!("dunno")
        }
    }

    fn score(&self) -> i32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win  => 6
        }
    }
}

fn main() {
    let input = get_input(2022);

//     let input = "\
// A Y
// B X
// C Z".to_string();

    let mut score = 0;

    for line in input.lines() {
        let opp = RPS::parse(line.chars().nth(0).unwrap());
        let me  = RPS::parse(line.chars().nth(2).unwrap());
        // println!("Score for {:?} @ {:?} = {}", me, opp, RPS::score_for_round(me, opp));
        score += RPS::score_for_round(me, opp);
    }

    println!("Total score: {score}");
}

