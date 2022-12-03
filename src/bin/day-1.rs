fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-1.txt")?;

    let mut acc = 0;
    let mut most_calories = 0;

    for line in input.lines() {
        match line.trim().parse::<i32>() {
            Ok(number) => acc += number,
            Err(_) => {
                if acc > most_calories {
                    most_calories = acc;
                }
                acc = 0;
            }
        }
    }

    println!("{most_calories}");

    Ok(())
}
