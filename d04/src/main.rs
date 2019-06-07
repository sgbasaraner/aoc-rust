use std::fs;
use std::collections::HashMap;
use std::ops::Range;

const PREFIX_INDEX: usize = 17;

fn main() {
    let input_sessions = parse_input_to_sessions();
    println!("{}", part_one(input_sessions.clone()));
    println!("{}", part_two(input_sessions));
}

#[derive(Clone)]
enum Event {
	BeginsShift(i32),
	WakesUp(i32),
	FallsAsleep(i32)
}

impl Event {
    fn parse_from_str(string: String) -> Event {
        let suffix = &string[PREFIX_INDEX..];

        if string.contains("Guard") {
            let id_str: String = suffix.chars().filter(|c| c.is_digit(10) ).collect();
            let id = id_str.parse::<i32>().unwrap();
            return Event::BeginsShift(id);
        }

        let prefix: String = string.chars().take(PREFIX_INDEX).collect();
        let min_str: String = prefix[(prefix.len() - 2)..].to_string();
        let min = min_str.parse::<i32>().unwrap();

        if suffix.contains("falls") {
            return Event::FallsAsleep(min);
        }
            
        Event::WakesUp(min)
    }
}

#[derive(Clone)]
struct Session {
    guard_id: i32,
    events: Vec<Event>
}

fn organize_logs_chronologically(input_lines: Vec<String>) -> Vec<String> {
    let mut copy = input_lines.clone();
    copy.sort_by(|a, b| {
        let first: String = a.chars().take(PREFIX_INDEX).collect();
        let second: String = b.chars().take(PREFIX_INDEX).collect();
        first.cmp(&second) 
    });
    copy
}

fn organize_sessions(organized_logs: Vec<String>) -> Vec<Session> {
    let mut sessions: Vec<Session> = Vec::new();
    let mut cur_id: Option<i32> = None;
    let mut cur_events: Vec<Event> = Vec::new();

    for log in organized_logs {
        let cur_event = Event::parse_from_str(log);
        match cur_event {
            Event::BeginsShift(id) => {
                if let Some(prev_id) = cur_id {
                    sessions.push(Session {guard_id: prev_id, events: cur_events.clone()});
                }
                cur_id = Some(id);
                cur_events.clear(); 
            },
            _ => cur_events.push(cur_event)
        }
    }

    sessions
}

fn get_sleep_detail_dicts(sessions: Vec<Session>) -> (HashMap<i32, i32>, HashMap<i32, Vec<Vec<(i32, i32)>>>) {
    let mut guards_sleep_times: HashMap<i32, i32> = HashMap::new();
    let mut guards_sleep_ranges: HashMap<i32, Vec<Vec<(i32, i32)>>> = HashMap::new();

    for session in sessions {
        let mut total_sleep_time = 0;
        let mut last_sleep = 0;
        let mut cur_times: Vec<(i32, i32)> = Vec::new();

        for event in session.events {
            match event {
                Event::FallsAsleep(min) => last_sleep = min,
                Event::WakesUp(min) => {
                    total_sleep_time += min - last_sleep;
                    cur_times.push((min, last_sleep));
                },
                _ => continue
            }
        }

        *guards_sleep_times.entry(session.guard_id).or_insert(0) += total_sleep_time;
        guards_sleep_ranges.entry(session.guard_id).or_insert(Vec::new()).push(cur_times);
    }

    (guards_sleep_times, guards_sleep_ranges)
}

fn find_max_minute_and_count(guard_id: i32, sleep_ranges: HashMap<i32, Vec<Vec<(i32, i32)>>>) -> (i32, i32) {
    let ranges: Vec<Range<i32>> = sleep_ranges.get(&guard_id).unwrap().into_iter()
        .flatten()
        .map(|t| Range{ start: t.1, end: t.0 } )
        .collect();

    let mut min_counts: HashMap<i32, i32> = HashMap::new();
    for range in ranges { 
        for i in range {
            *min_counts.entry(i).or_insert(0) += 1;
        }
    }

    let mut pairs: Vec<(i32, i32)> = min_counts.into_iter().map(|(k, v)| (k, v)).collect();
    pairs.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    if pairs.is_empty() {
        return (0, 0);
    }
    pairs[0]
}

fn find_most_sleeping_guard(sleep_times: HashMap<i32, i32>) -> i32 {
    let mut pairs: Vec<(i32, i32)> = sleep_times.into_iter().map(|(k, v)| (k, v)).collect();
    pairs.sort_by(|a, b| a.1.cmp(&b.1).reverse() );
    let most_id: i32 = pairs[0].0;
    most_id
}

fn part_one(sessions: Vec<Session>) -> i32 {
    let dicts = get_sleep_detail_dicts(sessions);
    let most_sleeping_guard = find_most_sleeping_guard(dicts.0);
    let max_min = find_max_minute_and_count(most_sleeping_guard, dicts.1).0;
    most_sleeping_guard * max_min
}

fn part_two(sessions: Vec<Session>) -> i32 {
    let ranges = get_sleep_detail_dicts(sessions).1;
    let guard_ids: Vec<i32> = ranges.iter().map(|(k, _v)| *k).collect();

    let mut max_win = 0;
    let mut max_count = 0;
    let mut winner_id = 0;

    for id in guard_ids {
        let tuple = find_max_minute_and_count(id, ranges.clone());
        if max_count < tuple.1 {
            max_count = tuple.1;
            max_win = tuple.0;
            winner_id = id;
        }
    }
    winner_id * max_win
}

fn parse_input_to_sessions() -> Vec<Session> {
    let contents = fs::read_to_string("src/input.in")
        .expect("Something went wrong reading the file");
    let lines = contents.lines().map(|s| s.to_string()).collect();
    organize_sessions(organize_logs_chronologically(lines))
}