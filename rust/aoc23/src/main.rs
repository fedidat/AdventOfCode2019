use std::fs;
use std::collections::BTreeMap;

#[derive(Clone)]
struct Computer{
    ip: usize,
    program: BTreeMap<usize, i64>,
    relbase: i64,
    input: Vec<i64>,
    id: i64
}

#[derive(Eq, PartialEq)]
pub enum StatusCode {
  Output(i64),
  Step,
  AwaitInput,
  Exit
}

impl Computer {
    fn new(program: BTreeMap<usize, i64>, input: Vec<i64>, id: i64) -> Computer {
        Computer { ip: 0, program: program, relbase: 0, input: input, id: id }
    }
    fn enqueue(&mut self, input: Vec<i64>) {
        self.input.append(&mut input.clone());
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
    fn step(&mut self) -> StatusCode {
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
                self.program.insert(op3 as usize, if !self.input.is_empty() { self.input.remove(0) } else { -1 });
                self.ip += 2;
                return StatusCode::AwaitInput;
            }
            4 => {
                let op1 = self.mode(1);
                self.ip += 2;
                return StatusCode::Output(op1);
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
                return StatusCode::Exit;
            }
            _ => { panic!() }
        };
        return StatusCode::Step;
    }
    fn execute(&mut self) -> StatusCode {
        let mut result;
        loop {
            result = self.step();
            if result != StatusCode::Step {
                return result;
            }
        }
    }
  pub fn next_output(&mut self) -> i64 {
    match self.execute() {
      StatusCode::Output(o) => o,
      _ => unreachable!("Computer did send output")
    }
  }
}

fn simulate(programvec: Vec<i64>, part1: bool) {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computers: Vec<Computer> = (0..50).map(|i| Computer::new(program.clone(), vec![i], i)).collect();
    let mut nat_packet: Option<(i64, i64)> = None;
    let mut prev_y: Option<i64> = None;
    loop {
        let mut is_idle = true;
        for i in 0..50 {
            if !computers[i].input.is_empty() {
                is_idle = false;
            }
            match computers[i].execute() {
                StatusCode::Output(addr) => {
                    let (x, y) = (computers[i].next_output(), computers[i].next_output());
                    if addr == 255 { //packet for NAT
                        if part1 {
                            println!("Y of packet 255 is {}", y);
                            return;
                        }
                        nat_packet = Some((x, y));
                    }
                    else {
                        computers[addr as usize].enqueue(vec![x,y]);
                    }
                },
                _ => {}
            }
        }
        if is_idle && nat_packet != None {
            let (x, y) = nat_packet.unwrap();
            if prev_y != None && prev_y.unwrap() == y {
                println!("Y of same NAT packet sent to 0 twice is {}", y);
                return;
            }
            prev_y = Some(y);
            computers[0].enqueue(vec![x,y]);
        }
    }
}

fn part1(programvec: Vec<i64>) {
    simulate(programvec, true);
}
fn part2(programvec: Vec<i64>) {
    simulate(programvec, false);
}

fn main() {
    let contents = fs::read_to_string("day23.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}