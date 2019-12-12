use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::HashSet;

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y :isize
}
impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x: x, y: y }
    }
    fn add(&mut self, direction: Point) -> Point {
        Point::new(self.x + direction.x, self.y + direction.y)
    }
}

fn run_robot(programvec: Vec<i64>, start_color: usize) -> (HashSet::<Point>, HashSet::<Point>) {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer = Computer::new(program);
    let directions = vec![Point::new(0,1),Point::new(1,0),Point::new(0,-1),Point::new(-1,0)];
    let (mut point, mut direction) = (Point::new(0,0), 0 as i64);
    let (mut grid, mut painted) = (HashSet::<Point>::new(), HashSet::<Point>::new());
    if start_color == 1 {
        grid.insert(point);
    }
    loop {
        let color = computer.execute(VecDeque::from(vec![grid.contains(&point) as i64]));
        match color {
            Some(0) => { grid.remove(&point); }
            Some(1) => { grid.insert(point); }
            _ => { break; }
        }
        painted.insert(point);

        let turn = computer.execute(VecDeque::from(vec![grid.contains(&point) as i64]));
        direction = (direction + turn.unwrap()*2 + 3) % 4;
        point = point.add(directions[direction as usize]);
    }
    return (grid, painted);
}

fn part1(program: Vec<i64>) {
    let (_, painted) = run_robot(program, 0);
    println!("painted {:?} tiles", painted.len());
}
fn part2(program: Vec<i64>) {
    let (grid, _) = run_robot(program, 1);
    let min_x = (*grid.iter().min_by_key(|pt| pt.x).unwrap()).x;
    let max_x = (*grid.iter().max_by_key(|pt| pt.x).unwrap()).x;
    let min_y = (*grid.iter().min_by_key(|pt| pt.y).unwrap()).y;
    let max_y = (*grid.iter().max_by_key(|pt| pt.y).unwrap()).y;
    for y in (min_y..max_y+1).rev() {
        for x in min_x..max_x+1 {
            print!("{}", if grid.contains(&Point::new(x,y)) { "#" } else { " " });
        }
        println!("");
    }
}

fn main() {
    let contents = fs::read_to_string("day11.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}