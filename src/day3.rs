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
    let regex =
        Regex::new(r"(?m)(do\(\)|don't\(\)|mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\))").unwrap();

    let (sum, _) =
        regex
            .captures_iter(input)
            .fold((0, true), |(mut sum, mut enabled), captures| {
                if let Some(matched) = captures.get(0) {
                    enabled = match matched.as_str() {
                        "do()" => true,
                        "don't()" => false,
                        _ => enabled,
                    };
                }
                if let (Some(num1), Some(num2)) = (captures.name("num1"), captures.name("num2")) {
                    if enabled {
                        sum += num1.as_str().parse::<i64>().unwrap()
                            * num2.as_str().parse::<i64>().unwrap();
                    }
                }
                (sum, enabled)
            });
    sum
}
pub fn answers() -> (i64, i64) {
    let input = include_str!("../input/day3.txt");
    (part1(input), part2(input))
}
