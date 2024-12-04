use regex::Regex;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn check_dirs(from: (usize, usize), direction: Direction, input: &Vec<Vec<char>>) {
    // let indices = [(Direction::Right, from..=from+3)];

    let (x, y) = from;

    // match direction {}

    // let isUp = up.iter().zip("XMAS".chars()).all(|(c, expected)| *c== expected);
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

fn part1(input: Vec<Vec<char>>) -> usize {
    let diag = Regex::new("X....M....A....S").unwrap();
    let mut count = 0;
    input.iter().enumerate().for_each(|(x, line)| {
        line.iter().enumerate().for_each(|(y, _)| {
            let expected = "XMAS";
            let square = range_2d((x, y), (4, 4), &input)
                .flatten()
                .collect::<String>();

            // let y = y as i64;
            // let x = x as i64;
            // let right = (x + 4, y);
            // let upright = (x + 4, y - 4);
            // let downright = (x + 4, y + 4);

            // let left = (x - 4, y);
            // let upleft = (x - 4, y - 4);
            // let downleft = (x - 4, y + 4);

            // let up = (x, y - 4);
            // let down = (x, y + 4);

            let right = range_2d((x as usize, y as usize), (4, 1), &input)
                .flatten()
                .collect::<String>();

            // let left = range_2d(
            //     (((x - 4) as usize).clamp(0, line.len()), y as usize),
            //     (4, 1),
            //     &input,
            // )
            // .flatten()
            // .zip(expected.chars().into_iter())
            // .all(|(c1, c2)| *c1 == c2);

            // let up = range_2d((x as usize, y as usize), (1, 4), &input)
            //     .flatten()
            //     .zip(expected.chars().into_iter())
            //     .all(|(c1, c2)| *c1 == c2);

            let down = range_2d(
                (x as usize, (y as usize).clamp(0, input.len())),
                (1, 4),
                &input,
            )
            .flatten()
            .collect::<String>();
            // let square_str = range_2d((x as usize, y as usize), (4, 4), &input)
            //     .flatten()
            //     .collect::<String>();
            let mut diagonal = Vec::new();
            if ((x + 4) as usize) < line.len() && y + 4 < input.len() {
                diagonal = vec![
                    input[x][y],
                    input[x + 1][y + 1],
                    input[x + 2][y + 2],
                    input[x + 3][y + 3],
                ]
            }
            if y == 0 {
                println!("{right} {down} {diagonal:?}")
            }
            let right = right == expected;
            let down = down == expected;
            let diag = diagonal.into_iter().collect::<String>() == expected;
            if right {
                count += 1;
            }
            if down {
                count += 1;
            }
            if diag {
                count += 1;
            }
            // if right == expected
            //     || down == expected
            //     || diagonal.into_iter().collect::<String>() == expected
            // {
            //     count += 1;
            // }
            // let square = range_2d((x as usize , y as usize), (4, 4), &input);
        });
    });
    count
}

fn part2() -> usize {
    0
}

pub fn answers() -> (usize, usize) {
    // let input = include_str!("../input/day4.txt");
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    // println!("{input}");
    let input = parse_input(input);
    // let input = vec![
    //     vec!['a', 'b', 'd', 'h'],
    //     vec!['c', 'd', 'x', 'w'],
    //     vec!['h', 'q', 'z', 'j'],
    //     vec!['c', 'r', 'e', 'b'],
    // ];
    // let (x, y) = (0usize, 0usize);

    // // let right = range_2d((0, 0), (4, 1), &input);

    // // input.iter().enumerate().for_each(|(x, line)| {
    // //     line.iter()
    // //         .enumerate()
    // //         .for_each(|(y, _)| println!("{:?}", range_2d((x, y), (1, 4), &input)));
    // // });

    // // check_dirs((0, 0), Direction::Down, input);
    // let left = range_2d((x, y), (1, 4), &input)
    //     .flatten()
    //     .collect::<String>();
    // println!("{left}");
    // let right = Regex::new("XMAS").unwrap();
    // let left = Regex::new("SAMX").unwrap();
    // let diag = Regex::new("X....M....A....S").unwrap();
    // let down = Regex::new("X...M...A...S...").unwrap();

    // let part1d =
    //     (0..input.len())
    //         .zip((0..input.len()).skip(16))
    //         .fold(0, |mut count, (start, end)| {
    //             let str = &input[start..end];

    //             if right.is_match(&str) {
    //                 count += 1;
    //             }

    //             if diag.is_match(&str) {
    //                 count += 1;
    //             }

    //             if down.is_match(&str) {
    //                 count += 1;
    //             }

    //             if left.is_match(&str) {
    //                 count += 1;
    //             }
    //             println!("{count}");
    //             count
    //         });

    (part1(input), part2())
    // (part1d, 0)
}
