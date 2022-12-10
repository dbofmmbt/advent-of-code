fn main() -> eyre::Result<()> {
    let input = std::fs::read_to_string("input/day-6.txt")?;

    let (idx, _) = input
        .chars()
        .enumerate()
        .find(|(idx, _)| all_different(&input[*idx..(*idx + 4)]))
        .unwrap();

    println!("{}", idx + 4);

    Ok(())
}

fn all_different(slice: &str) -> bool {
    for (idx, el) in slice.chars().enumerate() {
        for other in slice[(idx + 1)..].chars() {
            if el == other {
                return false;
            }
        }
    }
    true
}
