fn get_input() -> Result<Vec<Vec<u64>>, std::io::Error> {
    Ok(std::fs::read_to_string("input/day2.txt")?
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|iter| iter.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>())
}

fn part1(input: Vec<Vec<u64>>) -> usize {
    input
        .into_iter()
        .filter(|nums| {
            let is_increasing =
                nums.iter().next().unwrap_or(&0) < nums.iter().skip(1).next().unwrap_or(&0);
            nums.iter().zip(nums.iter().skip(1)).all(|(prev, next)| {
                ((prev < next) == is_increasing) && (1..=3).contains(&prev.abs_diff(*next))
            })
        })
        .count()
}

fn part2(input: Vec<Vec<u64>>) -> u64 {
    0
}

pub fn answers() -> Result<(usize, u64), std::io::Error> {
    Ok((part1(get_input()?), part2(get_input()?)))
}
