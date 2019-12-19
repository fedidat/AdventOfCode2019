use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;

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
    fn execute(&mut self, input: &mut VecDeque<i64>) -> Option<i64> {
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
    fn add(self, x: i64, y: i64) -> Point {
        Point { x: self.x + x, y: self.y + y}
    }
}

fn in_beam(point: Point, programvec: Vec<i64>)  -> bool {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let computer = Computer::new(program.clone());
    let coordinates = VecDeque::from(vec![point.x as i64, point.y]);
    return computer.clone().execute(&mut coordinates.clone()).unwrap() == 1;
}

fn part1(programvec: Vec<i64>) {
    let mut attracted = 0;
    for y in 0..50 {
        for x in 0..50 {
            if in_beam(Point::new(x, y), programvec.clone()) {
                attracted += 1;
            }
        }
    }
    println!("Number of points in beam in origin 50x50 square: {}", attracted);
}
    
fn part2(programvec: Vec<i64>) {
    let mut point = Point::new(0, 6); //rows 1,2,5 are blank because of the beam angle 
    loop {
        while !in_beam(point, programvec.clone()) { 
            point = point.add(1, 0); //got beam
        }
        if in_beam(point.add(0, -99), programvec.clone()) && in_beam(point.add(99, -99), programvec.clone()) {
            let result_point = point.add(0, -99);
            let result = result_point.x * 10000 + result_point.y;
            println!("Edge of the ship is {:?}, answer is: {}", result_point, result);
            return;
        }
        point = point.add(0, 1); //next row
    }
}

fn main() {
    let contents = fs::read_to_string("day19.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}