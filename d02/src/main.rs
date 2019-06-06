use std::fs;
use std::collections::HashMap;

fn main() {
    let lines = get_input_lines();
    println!("{}", part_one(lines.clone()));
    println!("{}", part_two(&lines).unwrap());
}

fn part_one(lines: Vec<String>) -> i32 {
    let mut two_c = 0;
    let mut three_c = 0;
    for line in lines {
        let mut counts: HashMap<char, u64> = HashMap::new();
        for ch in line.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        let vals: Vec<u64> = counts.iter().map(|(_k, v)| v.clone() ).collect();
        if vals.contains(&2) {
            two_c += 1;
        }
        if vals.contains(&3) {
            three_c += 1;
        }
    }
    return two_c * three_c;
}

fn part_two(lines: &Vec<String>) -> Option<String> {
    for l1 in lines {
        for l2 in lines {
            match get_common_chars(&l1, &l2) {
                Some(s) => { return Some(s); },
                _ => ()
            }
        }
    }
    return None;
}

fn get_common_chars(str1: &str, str2: &str) -> Option<String> {
    if str1 == str2 { return None; }

    let mut common_chars = String::new();
    let mut diff = 0;
    let len = str1.len();
    if str2.len() != len { return None; }
    
    for i in 0..len {
        if str1.chars().nth(i).unwrap() == str2.chars().nth(i).unwrap() {
            common_chars.push_str(&(str1.chars().nth(i).unwrap().to_string()));
        } else {
            diff += 1;
        }
        if diff > 1 { return None; }
    }
    
    Some(common_chars.to_string())
}

fn get_input_lines() -> Vec<String> {
    let contents = fs::read_to_string("src/input.in")
        .expect("Something went wrong reading the file");
    contents.lines().map( |x| x.to_string() ).collect()
}