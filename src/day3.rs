use regex::Regex;

fn part1(input: &str) -> i64 {
    let regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    regex
        .captures_iter(input)
        .map(|captures| {
            let (_, [num1, num2]) = captures.extract::<2>();
            num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let regex = Regex::new(r"(?U).*do\(\)").unwrap();
    println!("start");
    let first = regex
        .find_iter(input)
        .map(|matched| {
            println!("{}", matched.as_str());
            part1(matched.as_str())
        })
        .sum::<i64>();
    let regex = Regex::new(r"(?mU)do\(\).*don't\(\)").unwrap();
    println!("mid");
    let mut last_match: usize = 0;
    let mid = regex
        .find_iter(input)
        .map(|matched| {
            println!("{}", matched.as_str());
            last_match = matched.start();
            part1(matched.as_str())
        })
        .sum::<i64>();
    println!("last");
    let last = Regex::new(r"(?U)do\(\).*").unwrap();
    let last = last
        .find_iter(&input[last_match..input.len()])
        .map(|matched| {
            println!("{}", matched.as_str());
            last_match = matched.start();
            part1(matched.as_str())
        })
        .sum::<i64>();

    first + mid
}
pub fn answers() -> (i64, i64) {
    let input = include_str!("../input/day3.txt");
    (
        part1(input),
        part2(r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
    )
}
