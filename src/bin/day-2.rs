use std::str::FromStr;

use eyre::eyre;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum RoundResult {
    Victory,
    Draw,
    Loss,
}

impl RoundResult {
    pub fn value(&self) -> i32 {
        match self {
            RoundResult::Victory => 6,
            RoundResult::Draw => 3,
            RoundResult::Loss => 0,
        }
    }
}

impl Choice {
    pub fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl FromStr for RoundResult {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Loss),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Victory),
            _ => Err(eyre!("not a valid choice")),
        }
    }
}

impl FromStr for Choice {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(eyre!("not a valid choice")),
        }
    }
}

fn perform_round(theirs: Choice, result: RoundResult) -> i32 {
    use Choice::*;
    use RoundResult::*;
    let choice = match (&theirs, &result) {
        (Rock, Loss) => Scissors,
        (Rock, Draw) => Rock,
        (Rock, Victory) => Paper,
        (Paper, Loss) => Rock,
        (Paper, Draw) => Paper,
        (Paper, Victory) => Scissors,
        (Scissors, Loss) => Paper,
        (Scissors, Draw) => Scissors,
        (Scissors, Victory) => Rock,
    };

    choice.value() + result.value()
}

fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-2.txt")?;

    let score: i32 = input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let a = iter.next();
            let b = iter.next();
            (
                a.map(Choice::from_str).unwrap(),
                b.map(RoundResult::from_str).unwrap(),
            )
        })
        .map(|(a, b)| perform_round(a.unwrap(), b.unwrap()))
        .sum();

    println!("{score}");

    Ok(())
}
