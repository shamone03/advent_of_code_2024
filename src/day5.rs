use std::collections::HashMap;

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, Vec<Vec<usize>>) {
    let mut lines = input.lines();
    (
        lines.by_ref().take_while(|line| !line.is_empty()).fold(
            HashMap::<usize, Vec<usize>>::new(),
            |mut accum, line| {
                let mut split = line.split_terminator("|");
                let num1 = split.next().unwrap().parse().unwrap();
                let num2 = split.next().unwrap().parse().unwrap();
                accum
                    .entry(num1)
                    .and_modify(|vec| vec.push(num2))
                    .or_insert(vec![num2]);

                accum
            },
        ),
        lines
            .map(|line| {
                line.split_terminator(",")
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect(),
    )
}

fn part1(ordering: HashMap<usize, Vec<usize>>, pages: Vec<Vec<usize>>) -> usize {
    pages
        .iter()
        .enumerate()
        .filter(|(_, page)| {
            let mut prev = Vec::<usize>::new();
            let ret = page.iter().all(|page_num| {
                if ordering.contains_key(page_num) {
                    prev.push(*page_num);
                }

                // let curr = &ordering[page_num];

                let count = prev
                    .iter()
                    .filter(|num| page_num != *num)
                    .count();
                let out = if count == 0 {
                    println!("here2");
                    true
                } else {
                    prev.iter()
                        .filter(|num| page_num != *num)
                        .map(|indices| &ordering[indices])
                        .any(|should_be_after| should_be_after.contains(page_num))
                };
                if out {
                    // println!("{prev:?}");
                }
                out
                // true
            });
            // println!("{prev:?}");
            ret
        }).inspect(|(_, line)| println!("{line:?}"))
        .count()
}

pub fn answers() -> (usize, usize) {
    let input = r"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

61,13,29";
    let (ordering, pages) = parse_input(input);
    // println!("{:#?}", parse_input(input));
    (part1(dbg!(ordering), pages), 0)
}
