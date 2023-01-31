
// 23:30 start
// 23:53 part1 done
// 0:14 part2 done
// 1:08 improved

use input_downloader;

use std::collections::HashSet;


fn main() {
    let input = input_downloader::year(2022).example("\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
").get();

    let mut sum: u32 = 0;
    let mut trio = Vec::new();

    for line in input.lines() {
        let items = parse_items(line);
        trio.push(items);
        if trio.len() == 3 {
            let common1: HashSet<char> = trio[0].intersection(&trio[1])
                .copied().collect();
            let common = common1.intersection(&trio[2])
                .next().expect("no common item found!");

            sum += priority_from_char(*common) as u32;

            trio.clear();
        }
    }

    println!("sum is {sum}");
}

fn priority_from_char(c: char) -> u8 {
    match c {
        'a'..='z' => (c as u8) - ('a' as u8) + 1,
        'A'..='Z' => (c as u8) - ('A' as u8) + 27,
        _ => panic!("invalid priority char")
    }
}

fn parse_items(line: &str) -> HashSet<char> {
    let mut h = HashSet::new();
    for c in line.chars() {
        h.insert(c);
    }
    h
}
