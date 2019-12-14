use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::HashMap;

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
    x: i64,
    y :i64
}
impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }
}

fn part1(programvec: Vec<i64>) {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer = Computer::new(program);
    let mut blocks = 0;
    loop {
        match (0..3).map(|_| computer.execute(VecDeque::new())).collect::<Vec<Option<i64>>>()[2] {
            Some(2) => { blocks += 1; }
            None => { break; }
            _ => { }
        }
    }
    println!("Blocks: {}", blocks);
}

fn part2(mut programvec: Vec<i64>) {
    programvec[0] = 2;
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer = Computer::new(program);
    let mut grid: HashMap<Point,i64> = HashMap::new();
    let mut first_loop = true;
    let mut input = 0;
    let mut score = 0;
    loop {
        loop {
            let x = computer.execute(VecDeque::from(vec![input]));
            let y = computer.execute(VecDeque::new());
            let t = computer.execute(VecDeque::new());
            if t == None {
                println!("Final score: {}", score);
                return;
            }
            if x.unwrap() == -1 && y.unwrap() == 0 {
                score = t.unwrap();
                break;
            }
            grid.insert(Point::new(y.unwrap(), x.unwrap()), t.unwrap());
            if !first_loop && (t.unwrap() == 4 || t.unwrap() == 3) {
                break;
            }
        }
        first_loop = false;
        let ball_y = grid.iter().find(|&(_, v)| *v == 4).unwrap().0.y;
        let paddle_y = grid.iter().find(|&(_, v)| *v == 3).unwrap().0.y;
        input = if ball_y < paddle_y { -1 } else if ball_y > paddle_y { 1 } else { 0 };  
    }
}
fn main() {
    let contents = fs::read_to_string("day13.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}