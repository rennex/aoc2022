// 23:02 start
// 1:11 part 1 done. What a piece of shit!
// 1:20 part 2 done.

use input_downloader;
use std::collections::VecDeque;

fn main() {
    let input = input_downloader::year(2022).example(
"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
").get();

    let mut lines = input.lines();
    let mut stacks = parse_stacks(&mut lines);

    rearrange(&mut lines, &mut stacks);

    let tops: String = stacks[1..].into_iter().map(|s| s.iter().last().unwrap_or(&'_')).collect();

    println!("Result: {tops}");

}

fn parse_stacks<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<VecDeque<char>> {
    let mut stacks = Vec::new();

    for line in lines.by_ref() {
        // break on a line without "[crates]"
        if line.find('[').is_none() { break; }

        let mut stacknum = 1;
        let mut chars = line.chars();
        loop {
            let c = chars.next();

            match c {
                // end of line
                None => break,
                Some('[') => {
                    let label = chars.next().unwrap();
                    if chars.next() != Some(']') {
                        panic!("broken input line: {}", line);
                    }
                    chars.next(); // skip the trailing space

                    // if the stack by this number doesn't exist yet, create it
                    while stacks.len() <= stacknum {
                        stacks.push(VecDeque::new());
                    }

                    let stack = &mut stacks[stacknum];
                    stack.push_front(label);
                },
                // empty slot
                Some(' ') => for _ in 0..3 { chars.next(); },
                _ => panic!("garbage!")
            }

            stacknum += 1;
        }
    }

    // skip over an empty line
    lines.next();

    return stacks
}


fn rearrange<'a>(lines: &mut impl Iterator<Item=&'a str>, stacks: &mut Vec<VecDeque<char>>) {
    for line in lines.by_ref() {
        let ops: Vec<usize> = line.split(|c: char| !c.is_numeric())
            .filter_map(|c| c.parse().ok())
            .collect();

        if let [count, from, to] = ops[0..3] {
            println!("moving {count} from {from} to {to}");

            let mut storage = vec![];
            for _ in 0..count {
                let item = stacks[from].pop_back().unwrap();
                storage.push(item);
            }
            for _ in 0..count {
                let item = storage.pop().unwrap();
                stacks[to].push_back(item);
            }
        }
    }

}