use std::error::Error;

mod day3;
fn main() -> Result<(), Box<dyn Error>> {
    let (part1, part2) = day3::answers();
    println!("part 1: {part1}");
    println!("part 2: {part2}");

    Ok(())
}
