use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

type Coords = (i32, i32);

#[derive(Debug, Clone)]
struct Point {
    coords: Coords,
    closest_beacon_id: Option<String>,
    distance_to_beacon: i32,
    total_distance_to_all: i32
}

impl Point {
    fn update_with_beacon(&mut self, beacon_id: String, beacon_coords: Coords) {
        let new_dist = get_manhattan_distance(self.coords, beacon_coords);
        self.total_distance_to_all += new_dist;
        if new_dist == self.distance_to_beacon {
            self.closest_beacon_id = None;
            return;
        }
        if !(new_dist < self.distance_to_beacon) { return; }
        self.closest_beacon_id = Some(beacon_id);
        self.distance_to_beacon = new_dist;
    }
}

fn main() {
    let input = read_coord_tuples();
    println!("{:?}", part_one(input.clone()));
    println!("{:?}", part_two(input.clone()));
}

fn part_two(beacons: Vec<Coords>) -> usize {
    let points = detail_points(beacons.clone(), create_point_map(beacons.clone()));
    points.into_iter()
        .filter(|p| p.total_distance_to_all < 10000)
        .collect::<Vec<_>>()
        .len()
}

fn part_one(beacons: Vec<Coords>) -> i32 {
    let points = detail_points(beacons.clone(), create_point_map(beacons.clone()));

    let mut counts: HashMap<String, i32> = HashMap::new();
    for point in points.clone() {
        if point.closest_beacon_id.is_none() { continue; }
        let closest_id = point.closest_beacon_id.unwrap();
        *counts.entry(closest_id).or_insert(0) += 1;
    }
    let bottom_right_edge = get_bottom_right_edge(beacons.clone());
    let on_edge_ids: HashSet<String> = points.into_iter()
        .filter(|p| on_edge(bottom_right_edge, &p) && p.closest_beacon_id.is_some())
        .map(|p| p.closest_beacon_id.unwrap())
        .collect();

    let mut count_vec: Vec<_> = counts.iter().collect();
    count_vec.sort_unstable_by(|t1, t2| t1.1.cmp(t2.1));

    let filtered: Vec<_> = count_vec.into_iter()
        .filter(|t| !on_edge_ids.contains(t.0)).collect();

    *filtered.last().unwrap().1
}

fn on_edge(edge: Coords, point: &Point) -> bool {
    let xs = vec![0, edge.0];
    let ys = vec![0, edge.1];
    xs.contains(&point.coords.0) || ys.contains(&point.coords.1)
}

fn detail_points(beacon_coords: Vec<Coords>, points: Vec<Point>) -> Vec<Point> {
    let mut mut_points = points.clone();
    for beacon in beacon_coords {
        for i in 0..points.len() {
            mut_points[i].update_with_beacon(get_beacon_id(&beacon), beacon);
        }
    }
    
    mut_points
}

fn get_bottom_right_edge(beacon_coords: Vec<Coords>) -> Coords {
    let mut y_coords: Vec<i32> = beacon_coords.iter().map(|(_x, y)| *y).collect();
    let mut x_coords: Vec<i32> = beacon_coords.iter().map(|(x, _y)| *x).collect();
    y_coords.sort();
    x_coords.sort();
    let bottommost_point = y_coords.last().unwrap();
    let rightmost_point = x_coords.last().unwrap();

    (*rightmost_point, *bottommost_point)
}

fn create_point_map(beacon_coords: Vec<Coords>) -> Vec<Point> {
    let bottom_right_edge = get_bottom_right_edge(beacon_coords.clone());

    let mut vec: Vec<Point> = Vec::new();
    for x in 0..=bottom_right_edge.0 {
        for y in 0..=bottom_right_edge.1 {
            vec.push(Point {
                coords: (x, y),
                closest_beacon_id: None,
                distance_to_beacon: i32::max_value(),
                total_distance_to_all: 0
            })
        }
    }

    vec
}

fn get_beacon_id(beacon: &Coords) -> String {
    let strs = vec![beacon.0.to_string(), beacon.1.to_string()];
    strs.join(" ")
}

fn get_manhattan_distance(p1: Coords, p2: Coords) -> i32 {
    (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
}

fn read_coord_tuples() -> Vec<Coords> {
    let contents = fs::read_to_string("src/input.in").unwrap();

    contents.lines().map(|line| {
        let ints: Vec<i32> = line.split(", ").map(|x| x.parse::<i32>().unwrap()).collect();
        (ints[0], ints[1])
    }).collect()
}