
// 20:56 start
// 21:45? stop
// 1:30 last tweaks to input_downloader
// 1:43 tweaks finished, start solving
// 1:52 solved part 1
// 2:00 solved part 2 (and tweaked it more)

use input_downloader;

fn main() {
    let inputs = input_downloader::year(2022)
        .example("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        .example("bvwbjplbgvbhsrlpgdmjqwftvncz")
        .example("nppdvjthqldpwncqszvftbrmjlhg")
        .example("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        .example("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        .get_all();

    for input in inputs {
        solve(&input);
    }
}

fn solve(input: &String) {
    println!("result: {}", find_sop(input.as_bytes(), 14));
}

fn find_sop(data: &[u8], width: usize) -> usize {
    for i in width..data.len() {
        let mut bytes = Vec::from(&data[i-width..i]);
        let len_before = bytes.len();
        bytes.sort();
        bytes.dedup();
        if bytes.len() == len_before {
            return i;
        }
    }
    return 0;
}
