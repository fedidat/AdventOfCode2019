use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::char;

#[derive(Clone)]
struct Computer{
    ip: usize,
    program: BTreeMap<usize, i64>,
    relbase: i64,
    input: VecDeque<i64>
}

impl Computer {
    fn new(program: BTreeMap<usize, i64>, input: VecDeque<i64>) -> Computer {
        Computer { ip: 0, program: program, relbase: 0, input: input}
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
    fn execute(&mut self) -> Option<i64> {
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
                    self.program.insert(op3 as usize, self.input.pop_front().unwrap());
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

fn run_routine(programvec: Vec<i64>, routine: &str) {
    println!("Running with routing {}", routine);
    let input: VecDeque<i64> = routine.chars().map(|c| c as i64).collect();
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer = Computer::new(program.clone(), input);
    loop {
        let result: Option<i64> = computer.execute();
        if result == None {
            break;
        }
        if result.unwrap() > 256 {
            println!("{}", result.unwrap());
            break;
        }
        print!("{}", char::from_u32(result.unwrap() as u32).unwrap());
    }
}

fn part1(programvec: Vec<i64>) {
    //jump if hole anywhere ahead (3 tiles) and safe to jump (4th tile)
    let routine = "\
    NOT A T
    OR T J
    NOT B T
    OR T J
    NOT C T
    OR T J
    AND D J
    WALK
    ";
    run_routine(programvec, routine);
}

fn part2(programvec: Vec<i64>) {
    //jump if hole anywhere ahead (3 tiles) and safe to jump (4th tile)
    let routine = "\
    NOT A T
    OR T J
    NOT B T
    OR T J
    NOT C T
    OR T J
    AND D J
    AND E T
    OR H T
    AND T J
    RUN
    ";
    run_routine(programvec, routine);
}

fn main() {
    let contents = fs::read_to_string("day21.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}