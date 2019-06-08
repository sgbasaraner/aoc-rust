use std::fs;
use std::collections::HashSet;

fn main() {
    let nums = get_input_lines();
    println!("{}", part_one(nums.clone()));
    println!("{}", part_two(nums));
}

fn part_one(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |a, b| a + b)
}

fn part_two(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    let mut cur_freq = 0;
    let mut cycled = nums.into_iter().cycle();

    loop {
        if !set.insert(cur_freq) { break; }
        let num = cycled.next().unwrap();
        cur_freq += num;
    }

    cur_freq
}

fn get_input_lines() -> Vec<i32> {
    let contents = fs::read_to_string("src/input.in").unwrap();
    contents.lines().map({
        |x| x.replace("+", "").parse::<i32>().unwrap() })
    .collect()
}