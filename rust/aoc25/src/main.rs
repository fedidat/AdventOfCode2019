use std::fs;
use std::collections::BTreeMap;
use std::char;
use std::io;

struct Computer{
    ip: usize,
    program: BTreeMap<usize, i64>,
    relbase: i64,
    input: Vec<i64>
}

#[derive(Eq, PartialEq)]
pub enum StatusCode {
  Output(i64),
  Step,
  AwaitInput,
  Exit
}

impl Computer {
    fn new(program: BTreeMap<usize, i64>, input: Vec<i64>) -> Computer {
        Computer { ip: 0, program: program, relbase: 0, input: input }
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
                if self.input.is_empty() {
                    return StatusCode::AwaitInput;
                }
                self.program.insert(op3 as usize, if !self.input.is_empty() { self.input.remove(0) } else { -1 });
                self.ip += 2;
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
    fn next_line(&mut self) -> (String, bool) {
        let mut answer = String::new();
        loop {
            let result = self.execute();
            match result {
            StatusCode::Output(result) => {
                let ch = char::from_u32(result as u32).unwrap();
                answer.push(ch);
            },
            StatusCode::AwaitInput => {
                return (answer, false);
            },
            StatusCode::Step => {},
            _ => { return (answer, true); }
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

fn the_end(programvec: Vec<i64>) {
    /* This simulates the board, with optional shortcuts for directions. 
     * Good luck collecting all the items and trying all combinations...
     * For me that was:
     * - n, n, n, take hypercube
     * - s, take monolith
     * - s, e, e, take easter egg
     * - e, s, take oranament
     * - w, s, w
     * THIS IS IT! SEE YOU IN 2020!
     */
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer: Computer = Computer::new(program.clone(), vec![]);
    let answer = computer.next_line();
    let mut current = Point::new(0,0);
    println!("{}", answer.0);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input = match input.trim().as_ref() {
            "n" => "north",
            "s" => "south",
            "e" => "east",
            "w" => "west",
            _ => { input.as_ref() }
        }.to_string();
        let direction = match input.trim().as_ref() {
            "north" => { Point::new(0, 1) },
            "south" => { Point::new(0, -1) },
            "east" => { Point::new(1, 0) },
            "west" => { Point::new(-1, 0) },
            _ => { Point::new(0, 0) }
        };
        current = current.add(direction);
        println!("position {:?}",current);
        let mut input_values: Vec<i64> = input.trim().chars().map(|c| c as i64).collect();
        input_values.push('\n' as i64);
        computer.enqueue(input_values);
        let (answer, done) = computer.next_line();
        println!("{}", answer);
        if done {
            println!("See you in 2020!");
            return;
        }
    }
}

fn main() {
    let contents = fs::read_to_string("day25.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    the_end(program.clone());
}