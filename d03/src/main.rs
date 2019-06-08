use std::fs;
use std::collections::HashMap;

fn main() {
    let claims = get_input_claims();
    println!("{}", part_one(&claims));
    println!("{}", part_two(&claims));
}

struct Claim {
    id: String,
    left: i32,
    top: i32,
    width: i32,
    height: i32
}

impl Claim {
    fn using_string(string: String) -> Claim {
        let split: Vec<&str> = string.split(' ').collect();
        let coords_split: Vec<&str> = split[2].split(',').collect();
        let size_split: Vec<&str> = split[3].split('x').collect();

        let id = split[0].replace("#", "").to_string();
        
        let left = coords_split[0].parse::<i32>().unwrap();
        let top = coords_split[1].replace(":", "").parse::<i32>().unwrap();

        let width = size_split[0].parse::<i32>().unwrap();
        let height = size_split[1].parse::<i32>().unwrap();

        Claim {
            id: id,
            left: left,
            top: top,
            width: width,
            height: height
        } 
    }

    fn get_used_coords(&self) -> Vec<(i32, i32)> {
        let mut pairs = Vec::new();
        for i in 0..self.width {
            for j in 0..self.height {
                pairs.push((self.left + i, self.top + j));
            }
        }
        return pairs;
    }
}

fn create_coord_dict(claims: &Vec<Claim>) -> HashMap<(i32, i32), i32> {
    let mut dict: HashMap<(i32, i32), i32> = HashMap::new();
    let pairs: Vec<(i32, i32)> = claims.into_iter().map(|c| c.get_used_coords() ).flatten().collect();

    for pair in pairs {
        *dict.entry(pair).or_insert(0) += 1;
    }
    
    dict
}

fn part_one(claims: &Vec<Claim>) -> usize {
    let dict = create_coord_dict(claims);
    
    let filtered: Vec<i32> = dict.into_iter()
        .map ( |(_k, v)| v )
        .filter( |v| *v > 1 )
        .collect();

    filtered.len()
}

fn part_two(claims: &Vec<Claim>) -> String { 
    let dict = create_coord_dict(claims);
    let intact_pairs: Vec<(i32, i32)> = dict.into_iter()
        .filter( |(_k, v)| *v == 1)
        .map( |(k, _v)| k )
        .collect();

    claims.into_iter()
        .find(|c| c.get_used_coords().iter().all(|p| intact_pairs.contains(p) ))
        .unwrap().id.clone()
}

fn get_input_claims() -> Vec<Claim> {
    let contents = fs::read_to_string("src/input.in").unwrap();
    contents.lines().map( |x| Claim::using_string(x.to_string()) ).collect()
}