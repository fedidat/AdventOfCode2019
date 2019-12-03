use std::fs;
use std::collections::HashMap;

fn get_positions(wire: Vec<&str>) -> HashMap<(isize, isize), usize> {
    let mut last = (0, 0);
    let mut step = 0;
    let mut positions = HashMap::new();
    for (idx,direction) in wire.iter().enumerate() {
        let (dir, dist) = direction.split_at(1);
        let distance = dist.parse::<isize>().unwrap();
        let mut multiplier: (isize, isize);
        match dir {
            "U" => { multiplier = (0, 1); }
            "D" => { multiplier = (0, -1); }
            "R" => { multiplier = (1, 0); }
            "L" => { multiplier = (-1, 0); }
            _ => { panic!(); }
        }
        for i in 1..distance+1 {
            step += 1;
            if !positions.contains_key(&(last.0+i*multiplier.0, last.1+i*multiplier.1)) {
                positions.insert((last.0+i*multiplier.0, last.1+i*multiplier.1), step);
            }
        }
        last = (last.0 + distance*multiplier.0, last.1 + distance*multiplier.1);
    }
    return positions;
}

fn manhattan_from_origin((x, y): (isize, isize)) -> usize {
    return (x.abs() + y.abs()) as usize;
}

fn part1(w1pos: &HashMap<(isize, isize), usize>, w2pos: &HashMap<(isize, isize), usize>) {
    let mut min_dist = usize::max_value();
    for (x, _) in w1pos {
        if w2pos.contains_key(x) {
            if manhattan_from_origin(*x) < min_dist {
                min_dist = manhattan_from_origin(*x);
            }
        }
    }
    println!("Closest intersection is at a distance {:?}", min_dist);
}
fn part2(w1pos: &HashMap<(isize, isize), usize>, w2pos: &HashMap<(isize, isize), usize>) {
    let mut min_step = usize::max_value();
    for (x, y) in w1pos {
        if w2pos.contains_key(x) {
            if y + w2pos.get(x).unwrap() < min_step {
                min_step = y + w2pos.get(x).unwrap();
            }
        }
    }
    println!("Minimal number of steps is {}", min_step);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let wires: Vec<&str> = contents.split("\n").collect();
    let wire1: Vec<&str> = wires[0].split(",").collect();
    let wire2: Vec<&str> = wires[1].split(",").collect();
    println!("Content: \n{:?} \n{:?}", &wire1[1..5], &wire2[1..5]);
    let (w1pos, w2pos) = (get_positions(wire1), get_positions(wire2));
    part1(&w1pos, &w2pos);
    part2(&w1pos, &w2pos);
}