use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i32},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

type Crate = char;

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    delimited(tag("["), anychar, tag("]"))(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    separated_list1(
        tag(" "),
        alt((map(parse_crate, Some), map(parse_hole, |_| None))),
    )(i)
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    amount: i32,
}

fn pile_number(i: &str) -> IResult<&str, usize> {
    map(i32, |i| (i - 1) as usize)(i)
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), i32),
            preceded(tag(" from "), pile_number),
            preceded(tag(" to "), pile_number),
        )),
        |(amount, from, to)| Instruction { from, to, amount },
    )(i)
}

fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-5.txt")?;
    let mut lines = input.lines();

    let crates: Vec<_> = lines
        .by_ref()
        .map_while(|l| all_consuming(crate_line)(l).ok())
        .map(|(_, c)| c)
        .collect();

    let mut stacks = transpose_rev(crates);

    // remove empty line
    lines.next();

    let instructions = lines
        .flat_map(|l| all_consuming(instruction)(l).map(|(_, i)| i))
        .collect::<Vec<_>>();

    for instruction in instructions {
        let from = &mut stacks[instruction.from];
        let base = from.len() - instruction.amount as usize;
        let values = from.drain(base..).collect::<Vec<_>>();

        let to = &mut stacks[instruction.to];
        to.extend(values);
    }

    stacks
        .iter()
        .flat_map(|s| s.last())
        .for_each(|c| print!("{c}"));

    println!();

    Ok(())
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                // 👇
                .rev()
                .filter_map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
