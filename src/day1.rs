use std::collections::HashMap;

fn get_input() -> Result<(Vec<u64>, Vec<u64>), std::io::Error> {
    std::fs::read_to_string("input/day1.txt").map(|file| {
        file.lines()
            .map(|line| {
                let mut nums = line.split_ascii_whitespace();
                (
                    nums.next().unwrap().parse::<u64>().unwrap(),
                    nums.next().unwrap().parse::<u64>().unwrap(),
                )
            })
            .unzip()
    })
}

fn part1(left: &mut Vec<u64>, right: &mut Vec<u64>) -> u64 {
    left.sort();
    right.sort();
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum::<u64>()
}

fn part2(left: &Vec<u64>, right: &Vec<u64>) -> u64 {
    let freqs = right.iter().fold(HashMap::new(), |mut freqs, i| {
        freqs.entry(i).and_modify(|count| *count += 1).or_insert(1);
        freqs
    });

    left.iter()
        .map(|left| freqs.get(&left).map(|freq| freq * left).unwrap_or(0))
        .sum()
}

pub fn answers() -> Result<(u64, u64), std::io::Error> {
    let (mut left, mut right) = get_input()?;
    Ok((part1(&mut left, &mut right), part2(&mut left, &mut right)))
}
