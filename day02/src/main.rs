
// 1:02 start (31.1.2023)
// 1:34 part 1 done
// 1:46 part 2 done

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
            'A' => Rock,
            'B' => Paper,
            'C' => Scissors,
            _ => panic!("invalid char")
        }
    }
}

#[derive(PartialEq,Debug,Clone,Copy)]
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
            (Rock, Scissors)    => Win,
            (Paper, Rock)       => Win,
            (Scissors, Paper)   => Win,
            _                   => Lose
        }
    }

    fn score(&self) -> i32 {
        match self {
            Lose => 0,
            Draw => 3,
            Win  => 6
        }
    }

    fn parse(c: char) -> Outcome {
        match c {
            'X' => Lose,
            'Y' => Draw,
            'Z' => Win,
            _ => panic!("invalid char")
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
    let my_choices = [Rock, Paper, Scissors];

    for line in input.lines() {
        let opp = RPS::parse(line.chars().nth(0).unwrap());
        let outcome = Outcome::parse(line.chars().nth(2).unwrap());
        let mut me = Rock;

        for choice in my_choices {
            if Outcome::of(choice, opp) == outcome {
                me = choice;
                break;
            }
        }
        score += RPS::score_for_round(me, opp);
    }

    println!("Total score: {score}");
}

