// 7:22 aloitus 29.1.2023
// 8:11 eka osa ratkaistu (jäin selailee rustin dokkareita)
// 8:19 toka osa ratkaistu! :)

use std::fs;

#[derive(Debug)]
struct Elf {
    items: i32,
    calories: i32
}

impl Elf {
    fn new() -> Elf {
        Elf { items: 0, calories: 0 }
    }
}

fn main() {
    let input = fs::read_to_string("input").expect("can't read 'input'");

    let mut elf = Elf::new();
    let mut elves = vec![];

    for line in input.lines() {
        match line.parse::<i32>() {
            Ok(n) => {
                elf.items += 1;
                elf.calories += n;
            },
            Err(_) => {
                //println!("{:?}!", elf);
                elves.push(elf);
                elf = Elf::new();
            }
        }
    }
    elves.sort_by_key(|e| e.calories);
    println!("found {} elves", elves.len());
    println!("most calories = {}", elves.last().expect("zero elves!").calories);

    let len = elves.len();
    let (e1, e2, e3) = (&elves[len-1], &elves[len-2], &elves[len-3]);
    println!("{:?}, {:?}, {:?}", e1, e2, e3);
    println!("total: {}", e1.calories + e2.calories + e3.calories);
}
