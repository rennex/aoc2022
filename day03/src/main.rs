
// 23:30 start
// 23:53 part1 done
// 0:14 part2 done

use input_downloader;

use std::collections::HashSet;


fn main() {
    let input = input_downloader::year(2022).example("\
vJrwpWtwJgWr hcsFMMfFFhFp
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
            let common_str = trio[0].intersection(&trio[1]).collect::<String>();
            let common_perkele = parse_items(&common_str);
            let common = common_perkele.intersection(&trio[2])
                .next().expect("no common item found!");

            println!("Found badge: {common}");
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

fn parse_line(line: &str) -> (HashSet<char>, HashSet<char>) {
    let split_pos = line.len() / 2;
    let compartment1 = &line[..split_pos];
    let compartment2 = &line[split_pos..];
    let items1 = parse_items(compartment1);
    let items2 = parse_items(compartment2);
    (items1, items2)
}