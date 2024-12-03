fn get_input() -> Result<Vec<Vec<u64>>, std::io::Error> {
    // let input = r#"6 7 3 2 1"#;
    // Ok(input
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
                let safe =
                    ((prev < next) == is_increasing) && (1..=3).contains(&prev.abs_diff(*next));

                safe
            })
        })
        .count()
}

fn part2(input: Vec<Vec<u64>>) -> usize {
    input
        .into_iter()
        .filter(|nums| {
            let is_valid = |prev: u64, next: u64, is_increasing: bool| {
                ((prev < next) == is_increasing) && (1..=3).contains(&prev.abs_diff(next))
            };
            // let mut removed = false;

            [
                (
                    nums.iter().skip(0),
                    nums.iter().skip(1),
                    nums.iter().skip(0).next().unwrap_or(&0)
                        < nums.iter().skip(1).next().unwrap_or(&0),
                    false,
                ),
                (
                    nums.iter().skip(1),
                    nums.iter().skip(2),
                    nums.iter().skip(1).next().unwrap_or(&0)
                        < nums.iter().skip(2).next().unwrap_or(&0),
                    true,
                )
            ]
            .into_iter()
            .any(|(mut start, mut forward, is_increasing, mut removed)| {
                let mut ret = true;
                while let (Some(prev), Some(next)) = (start.next(), forward.next()) {
                    let safe = is_valid(*prev, *next, is_increasing);
                    println!("prev:{prev}, next:{next}, {is_increasing}");
                    if !safe {
                        match removed {
                            true => {
                                ret = false;
                                break;
                            }
                            false => {
                                removed = true;
                                //6 7 3 2 1
                                // let mut s = forward.clone();
                                let mut prev = start.clone();
                                let mut next = forward.clone();

                                // compare with next or compare with or prev

                            }
                        }
                    }
                }
                ret
            })
            // ret
        })
        .count()
}

pub fn answers() -> Result<(usize, usize), std::io::Error> {
    Ok((part1(get_input()?), part2(get_input()?)))
}
