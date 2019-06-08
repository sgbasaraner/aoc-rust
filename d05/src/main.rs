use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let input = parse_input_to_char_arr();
    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input));
}

fn part_one(chars: Vec<char>) -> usize {
    let result: Vec<char> = react(chars);
    result.len()
}

fn part_two(chars: Vec<char>) -> usize {
    let all_units = get_all_lowercased_units(&chars);
    let mut unit_map: HashMap<char, Vec<char>> = HashMap::new();
    for unit in all_units { 
        let unit_removed = remove_unit(chars.clone(), unit);
        let fully_reacted = react(unit_removed);
        unit_map.insert(unit, fully_reacted);
    }

    let mut vals: Vec<Vec<char>> = unit_map.iter().map(|(_k, v)| v.clone()).collect();
    vals.sort_by(|v1, v2| v1.len().cmp(&v2.len()));
    vals[0].len()
}

fn remove_unit(chars: Vec<char>, lowercase_unit: char) -> Vec<char> {
    chars.into_iter()
        .filter(|c| !(*c == lowercase_unit || c.to_ascii_lowercase() == lowercase_unit))
        .collect()
}

fn get_all_lowercased_units(chars: &Vec<char>) -> HashSet<char> {
    let set: HashSet<char> = HashSet::from_iter(chars.iter().cloned());
    set.into_iter().filter(|c| c.is_ascii_lowercase()).collect()
}

fn react(chars: Vec<char>) -> Vec<char> {
    let mut mut_chars = chars.clone();
    for (i, ch) in chars.iter().enumerate() {

        if i + 1 == chars.len() {
            return chars;
        }

        let next = chars[i + 1];

        if compare_cases(&ch, &next) {
            for _ in 0..2 { mut_chars.remove(i); }
            return react(mut_chars);
        }
    }
    return chars;
}

fn compare_cases(ch1: &char, ch2: &char) -> bool {
    if ch1.is_ascii_lowercase() {
        return ch1.to_ascii_uppercase() == *ch2;
    } else {
        return ch1.to_ascii_lowercase() == *ch2;
    }
}

fn parse_input_to_char_arr() -> Vec<char> {
    let contents = fs::read_to_string("src/input.in").unwrap();
    contents.chars().collect()
}