use regex::Regex;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn range_2d(
    from: (usize, usize),
    size: (usize, usize),
    input: &Vec<Vec<char>>,
) -> impl Iterator<Item = impl Iterator<Item = &char>> {
    let (x, y) = from;
    let (x_size, y_size) = size;
    input
        .iter()
        .skip(y)
        .take(y_size)
        .map(move |line| line.iter().skip(x).take(x_size))
}

fn part1(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, _)| {
            let expected = "XMAS";
            let reverse = "SAMX";

            if (0..input.len()).contains(&(y + 3)) {
                if (0..line.len()).contains(&(x + 3)) {
                    let diagonal = format!(
                        "{}{}{}{}",
                        input[y][x],
                        input[y + 1][x + 1],
                        input[y + 2][x + 2],
                        input[y + 3][x + 3],
                    );

                    if diagonal == expected || diagonal == reverse {
                        count += 1;
                    }
                }
                if x >= 3 {
                    let diagonal = format!(
                        "{}{}{}{}",
                        input[y][x],
                        input[y + 1][x - 1],
                        input[y + 2][x - 2],
                        input[y + 3][x - 3],
                    );

                    if diagonal == expected || diagonal == reverse {
                        count += 1;
                    }
                }
            }

            if (0..line.len()).contains(&(x + 3)) {
                let right = range_2d((x, y), (4, 1), &input)
                    .flatten()
                    .collect::<String>();
                if right == expected || right == reverse {
                    count += 1;
                }
            }

            let down = range_2d((x, y), (1, 4), &input)
                .flatten()
                .collect::<String>();
            if down == expected || down == reverse {
                count += 1;
            }
        });
    });
    count
}

fn part2(input: &Vec<Vec<char>>) -> usize {
    let matches = [
        Regex::new("M.S.A.M.S").unwrap(),
        Regex::new("S.S.A.M.M").unwrap(),
        Regex::new("S.M.A.S.M").unwrap(),
        Regex::new("M.M.A.S.S").unwrap(),
    ];
    let mut count = 0;
    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, _)| {
            let square = range_2d((x, y), (3, 3), &input)
                .flatten()
                .collect::<String>();
            count += matches
                .iter()
                .filter(|regex| regex.is_match(&square))
                .count()
        });
    });
    count
}

pub fn answers() -> (usize, usize) {
    let input = include_str!("../input/day4.txt");
    let input = parse_input(input);

    (part1(&input), part2(&input))
}
