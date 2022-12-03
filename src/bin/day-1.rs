fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-1.txt")?;

    let mut acc = 0;

    let mut elves: Vec<i32> = Vec::new();

    for line in input.lines() {
        match line.trim().parse::<i32>() {
            Ok(number) => acc += number,
            Err(_) => {
                elves.push(acc);
                acc = 0;
            }
        }
    }

    elves.sort();

    let top_calories: i32 = elves.iter().rev().take(3).sum();

    println!("{top_calories}");

    Ok(())
}
