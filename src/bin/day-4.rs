use eyre::bail;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline},
    combinator::map,
    multi::separated_list0,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Input {
    pairs: Vec<Pair>,
}

type Pair = (Assignment, Assignment);

type Assignment = (i32, i32);

fn assignment(input: &str) -> IResult<&str, Assignment> {
    separated_pair(i32, tag("-"), i32)(input)
}

fn pair(input: &str) -> IResult<&str, Pair> {
    separated_pair(assignment, tag(","), assignment)(input)
}

fn input_parser(input: &str) -> IResult<&str, Input> {
    map(separated_list0(newline, pair), |v| Input { pairs: v })(input)
}

fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-4.txt")?;

    let Ok((_, input)) = input_parser(&input) else {
        bail!("couldn't parse input!");
    };

    let mut counter = 0;

    for ((a, b), (c, d)) in input.pairs {
        if (c >= a && c <= b) || (a >= c && a <= d) {
            counter += 1;
        }
    }

    println!("{counter}");

    Ok(())
}
