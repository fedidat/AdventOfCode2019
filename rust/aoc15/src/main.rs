use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::char;

#[derive(Clone)]
struct Computer{
    ip: usize,
    program: BTreeMap<usize, i64>,
    relbase: i64,
}

impl Computer {
    fn new(program: BTreeMap<usize, i64>) -> Computer {
        Computer { ip: 0, program: program, relbase: 0}
    }
    fn mode(&mut self, shift: usize) -> i64 {
        let modes_num = self.program.get(&self.ip).unwrap_or(&0) / 10_i64.pow(shift as u32 + 1) % 10;
        match modes_num {
            0 => { *(self.program.get(&(*(self.program.get(&(self.ip + shift)).unwrap_or(&0)) as usize)).unwrap_or(&0)) }
            1 => { *(self.program.get(&(self.ip + shift)).unwrap()) }
            2 => { *(self.program.get(&((*(self.program.get(&(self.ip + shift)).unwrap_or(&0)) + self.relbase) as usize)).unwrap_or(&0)) }
            _ => { panic!() }
        }
    }
    fn pos_mode(&mut self, shift: usize) -> i64 {
        let modes_num = self.program.get(&self.ip).unwrap_or(&0) / 10_i64.pow(shift as u32 + 1) % 10;
        match modes_num {
            0 => { *(self.program.get(&(self.ip + shift)).unwrap()) }
            2 => { (*(self.program.get(&(self.ip + shift)).unwrap_or(&0)) + self.relbase) as i64 }
            _ => { panic!() }
        }
    }
    fn execute(&mut self, mut input: VecDeque<i64>) -> Option<i64> {
        loop {
            match self.program.get(&self.ip).unwrap()%100 {
                1 => {
                    let (op1, op2, op3) = (self.mode(1), self.mode(2), self.pos_mode(3));
                    self.program.insert(op3 as usize, op1 + op2);
                    self.ip += 4;
                },
                2 => {
                    let (op1, op2, op3) = (self.mode(1), self.mode(2), self.pos_mode(3));
                    self.program.insert(op3 as usize, op1 * op2);
                    self.ip += 4;
                }
                3 => {
                    let op3 = self.pos_mode(1);
                    self.program.insert(op3 as usize, input.pop_front().unwrap());
                    self.ip += 2;
                }
                4 => {
                    let op1 = self.mode(1);
                    self.ip += 2;
                    return Some(op1);
                }
                5 => {
                    let (op1, op2) = (self.mode(1), self.mode(2));
                    self.ip = if op1 != 0 { op2 as usize } else { self.ip + 3 }
                }
                6 => {
                    let (op1, op2) = (self.mode(1), self.mode(2));
                    self.ip = if op1 == 0 { op2 as usize } else { self.ip + 3 }
                }
                7 => {
                    let (op1, op2, op3) = (self.mode(1), self.mode(2), self.pos_mode(3));
                    self.program.insert(op3 as usize, if op1 < op2 { 1 } else { 0 });
                    self.ip += 4;
                }
                8 => {
                    let (op1, op2, op3) = (self.mode(1), self.mode(2), self.pos_mode(3));
                    self.program.insert(op3 as usize, if op1 == op2 { 1 } else { 0 });
                    self.ip += 4;
                }
                9 => {
                    let op1 = self.mode(1);
                    self.ip += 2;
                    self.relbase += op1;
                }
                99 => {
                    return None;
                }
                _ => { panic!() }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i64,
    y :i64
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }
    fn add(self, point: Point) -> Point {
        return Point::new(self.x + point.x, self.y + point.y);
    }
}

fn visit_neighbors(computer: Computer, point: Point, mut visited: &mut HashMap<Point,char>) {
    for i in 1..5 {
        let neighbor = match i {
            1 => Point::new(point.x, point.y + 1),
            2 => Point::new(point.x, point.y - 1),
            3 => Point::new(point.x - 1, point.y),
            4 => Point::new(point.x + 1, point.y),
            _ => panic!()
        };
        visit_neighbor(computer.clone(), neighbor, i, &mut visited);
    }
}

fn visit_neighbor(mut computer: Computer, point: Point, direction: i64, mut visited: &mut HashMap<Point,char>) {
    let result = computer.execute(VecDeque::from(vec![direction]));
    let block = match result.unwrap() {
        0 => '#',
        1 => '.',
        2 => 'O',
        _ => panic!()
    };
    visited.insert(point, block);
    if block == '#' {
        return;
    } 
    for i in 1..5 {
        let neighbor = match i {
            1 => Point::new(point.x, point.y + 1),
            2 => Point::new(point.x, point.y - 1),
            3 => Point::new(point.x - 1, point.y),
            4 => Point::new(point.x + 1, point.y),
            _ => panic!()
        };
        if !visited.contains_key(&neighbor) {
            visit_neighbor(computer.clone(), neighbor, i, &mut visited);
        }
    }
}

fn bfs(mut grid: &mut HashMap<Point, char>, point: Point, end: bool) -> usize {
    let mut distance = 0;
    let mut items = vec![point];
    loop {
        let mut new_items: Vec<Point> = Vec::new();
        for i in items {
            for p in vec![Point::new(0,1), Point::new(0,-1), Point::new(1,0), Point::new(-1,0)] {
                match grid.get(&i.add(p)).unwrap() {
                    'O' => { 
                        if end { 
                            new_items.push(i.add(p)); 
                        } else { 
                            return distance + 1; 
                        } 
                    }
                    '.' => { new_items.push(i.add(p)); }
                    _ => {}
                }
            }
            grid.insert(i, char::from_digit((distance%10) as u32, 10).unwrap());
        }
        if new_items.is_empty() {
            break;
        }
        items = new_items.clone();
        distance += 1;
    }
    return distance;
}

fn part1(programvec: Vec<i64>) {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let computer = Computer::new(program);
    let mut grid: HashMap<Point,char> = HashMap::new();
    visit_neighbors(computer, Point::new(0,0), &mut grid);
    println!("oxygen is {} tiles away", bfs(&mut grid, Point::new(0,0), false));
}
fn part2(programvec: Vec<i64>) {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let computer = Computer::new(program);
    let mut grid: HashMap<Point,char> = HashMap::new();
    visit_neighbors(computer, Point::new(0,0), &mut grid);
    let oxygen_position = *(grid.iter().find(|&(_, v)| *v == 'O').unwrap().0);
    println!("ship filled after {} minutes", bfs(&mut grid, oxygen_position, true));
}

fn main() {
    let contents = fs::read_to_string("day15.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}