fn main() -> eyre::Result<()> {
    let mut input = std::fs::read("input/day-3.txt")?;

    let lines: Vec<&mut [u8]> = input.split_mut(|&it| it == b'\n').collect();

    let priorities: i32 = lines
        .chunks(3)
        .flat_map(|chunk| {
            chunk[0].iter().find(|el| {
                chunk[1]
                    .iter()
                    .find(|other| el == other)
                    .and_then(|_| chunk[2].iter().find(|other| el == other))
                    .is_some()
            })
        })
        .map(|el| -> i32 {
            if el >= &b'a' {
                el - b'a' + 1
            } else {
                el - b'A' + 1 + 26
            }
            .into()
        })
        .sum();

    println!("{priorities}");
    Ok(())
}
