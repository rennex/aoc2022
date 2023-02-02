
// 22:09 start
// 22:49 part1 done

use input_downloader;

use std::ops::RangeInclusive;

fn main() {
    let input = input_downloader::year(2022).example("\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
").get();

    let mut count = 0;

    for line in input.lines() {
        let mut ranges = line.split(',');

        let l = parse_range(ranges.next().unwrap());
        let r = parse_range(ranges.next().unwrap());

        if overlap(l, r) {
            count += 1;
        }
    }

    println!("count = {count}");
}

fn parse_range(range_txt: &str) -> RangeInclusive<i32> {
    let nums: Vec<_> = range_txt.split('-')
        .map(|num| {
            num.parse::<i32>().unwrap()
        })
        .collect();

    nums[0]..=nums[1]
}

fn overlap(l: RangeInclusive<i32>, r: RangeInclusive<i32>) -> bool {
    // println!("Comparing {:?} and {:?}", &l, &r);
    let c1 = l.start().cmp(r.start()) as i8;
    let c2 = l.end().cmp(r.end()) as i8;

    // println!("results: {c1} * {c2}");

    let res = c1*c2 != 1;

    // println!("= {res}");
    res
}