use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y :isize
}
impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
    fn add(self, point: Point) -> Point {
        return Point::new(self.x + point.x, self.y + point.y);
    }
    fn sub(self, point: Point) -> Point {
        return Point::new(self.x - point.x, self.y - point.y);
    }
}

#[derive(Clone, Hash, Debug, Eq, PartialEq)]
struct State {
    position: Vec<char>,
    obtained_keys: Vec<char>,
} 

fn bfs(path: &HashSet<Point>, doors: &HashMap<Point, char>, from: &Point, to: &Point) -> Option<(usize, Vec<char>)> {
    let mut queue: VecDeque<(Point, usize, Vec<char>)> = VecDeque::from(vec![(from.clone(), 0, Vec::new())]);
    let mut visited: HashSet<Point> = HashSet::from_iter(vec![from.clone()]);
    while !queue.is_empty() {
        let (current_pos, steps, necessary) = queue.pop_front().unwrap();
        if &current_pos == to {
            return Some((steps, necessary));
        }
        let mut next_necessary = necessary.clone();
        if doors.contains_key(&current_pos) {
            next_necessary.push(doors.get(&current_pos).unwrap().clone());
        }
        let neighbors = [current_pos.sub(Point::new(1, 0)), current_pos.add(Point::new(1, 0)),
                current_pos.sub(Point::new(0, 1)), current_pos.add(Point::new(0, 1))];
        let accessible_neighbors: Vec<Point> = neighbors.iter()
            .filter(|neighbor| path.contains(neighbor))
            .map(|neighbor| neighbor.clone())
            .collect();
        for neighbor in accessible_neighbors {
            if !visited.contains(&neighbor) {
                queue.push_back((neighbor, steps + 1, next_necessary.clone()));
                visited.insert(neighbor.clone());
            }
        }
    }
    None
}
fn load(input: &String) -> (HashMap<(char, char), (usize, Vec<char>)>, usize) {
    let path: HashSet<Point> = input.split('\n').enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate()
                .filter(|(_, c)| *c != '#')
                .map(|(x, _)| {
                    Point::new(x as isize, y as isize)
                })
                .collect::<HashSet<Point>>()
        })
        .collect();
    let not_key_door: HashMap<Point, char> = input.split('\n').enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .filter(|(_, c)| !vec!['#', '.', '@'].contains(c))
                .map(|(x, c)| (Point::new(x as isize, y as isize), c))
                .collect::<Vec<(Point, char)>>())
        .collect();
    let keys: HashMap<Point, char> = not_key_door.clone()
                .into_iter()
                .filter(|(_, c)| c.to_lowercase().next().unwrap() == *c)
                .map(|(p, c)| (p, c.to_uppercase().next().unwrap()))
                .collect();
    let doors: HashMap<Point, char> = not_key_door.clone()
                .into_iter()
                .filter(|(_, c)| c.to_uppercase().next().unwrap() == *c)
                .collect();
    let numbered_positions: HashMap<Point, char> = input.split('\n').enumerate()
        .flat_map(|(y, line)|
            line.chars().enumerate()
                .filter(|(_, c)| *c == '@')
                .map(|(x, _)| Point::new(x as isize, y as isize))
                .collect::<Vec<Point>>())
        .enumerate()
        .map(|(i, point)| (point.clone(), i as u8 as char))
        .collect();
    (keys.clone().into_iter()
         .chain(numbered_positions)
         .flat_map(|(point_one, key_one)|
             keys.iter().filter_map(|(point_two, key_two)| {
                 match bfs(&path, &doors, &point_one, point_two) { //call BFS to get distance to each key
                     Some(result) => Some(((key_one.clone(), key_two.clone()), result)),
                     None => None
                 }
             })
             .collect::<Vec<((char, char), (usize, Vec<char>))>>())
         .collect(), keys.len())
}

fn minimum_steps(state: &State, distances: &HashMap<(char, char), (usize, Vec<char>)>, num_keys: usize, 
        previous_states: &mut HashMap<State, usize>) -> usize {
    if num_keys == state.obtained_keys.len() {
        return 0;
    }
    if previous_states.contains_key(state) {
        return previous_states.get(state).unwrap().clone();
    }
    let mut min_steps = std::usize::MAX;
    let next_stops = distances.iter()
        .filter(|((from, to), (_, required_keys))| !state.obtained_keys.contains(to) 
                && state.position.contains(from) && required_keys.iter().find(|key| !state.obtained_keys.contains(key)).is_none())
        .map(|((from, to), (steps, _))| (state.position.iter().position(|i| i == from).unwrap(), to, steps));
    for (robot, next_stop, steps) in next_stops {
        let mut new_obtained_keys: Vec<char> = state.obtained_keys.iter().chain(vec![next_stop]).cloned().collect();
        new_obtained_keys.sort();
        let new_state = State {
            obtained_keys: new_obtained_keys,
            position: state.position.clone().iter().enumerate().map(|(i, at)| if i == robot { next_stop.clone() } else { at.clone() }).collect(),
        };
        min_steps = min_steps.min(minimum_steps(&new_state, distances, num_keys, previous_states) + steps.clone());
    }
    previous_states.insert(state.clone(), min_steps);
    min_steps
}

fn solve(input: &str) -> usize {
    let (distances, num_keys) = load(&input.to_string());
    let state = State {
        position: vec![0 as char, 1 as char, 2 as char, 3 as char],
        obtained_keys: Vec::new(),
    };
    minimum_steps(&state, &distances, num_keys, &mut HashMap::new())
}
fn part1() {
    let input = fs::read_to_string("day18-1.txt").unwrap();
    println!("with one access: {:?}", solve(&input));
}
fn part2() {
    let input = fs::read_to_string("day18-2.txt").unwrap();
    println!("with four accesses: {:?}", solve(&input));
}

fn main() {
    part1();
    part2();
}