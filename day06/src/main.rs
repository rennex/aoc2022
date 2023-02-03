
// 20:56 start
// 21:45? stop
// 1:30 last tweaks to input_downloader
// 1:43 tweaks finished, start solving
// 1:52 solved part 1


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
    println!("result: {}", find_sop(input.as_bytes()));
}

fn find_sop(data: &[u8]) -> usize {
    for i in 3..data.len() {
        let mut bytes = Vec::from(&data[i-3..=i]);
        bytes.sort();
        bytes.dedup();
        if bytes.len() == 4 {
            return i+1;
        }
    }
    return 0;
}
