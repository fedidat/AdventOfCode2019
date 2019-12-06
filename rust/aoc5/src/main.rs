use std::fs;

fn get_modes(modes_num: usize) -> Vec<bool> {
    (0..3).map(|i| {(modes_num / 10_usize.pow(i) % 10_usize.pow(i+1)) == 1}).collect()
}

fn execute(mut program: Vec<isize>, input: isize) {
    let mut i: usize = 0;
    loop {
        let modes: Vec<bool> = get_modes((program[i]/100) as usize);
        match program[i]%100 {
            1 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                let position = program[i+3] as usize;
                program[position] = op1 + op2;
                i += 4;
            },
            2 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                let position = program[i+3] as usize;
                program[position] = op1 * op2;
                i += 4;
            }
            3 => {
                let position = program[i+1] as usize;
                program[position] = input;
                i += 2;
            }
            4 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                println!("Output {}", op1);
                i += 2;
            }
            5 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                i = if op1 != 0 { op2 as usize } else { i + 3 }
            }
            6 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                i = if op1 == 0 { op2 as usize } else { i + 3 }
            }
            7 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                let position = program[i+3] as usize;
                program[position] = if op1 < op2 { 1 } else { 0 };
                i += 4;
            }
            8 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                let op2 = if modes[1] {program[i+2]} else {program[program[i+2] as usize]};
                let position = program[i+3] as usize;
                program[position] = if op1 == op2 { 1 } else { 0 };
                i += 4;
            }
            99 => {
                break
            }
            _ => { panic!() }
        }
    }
}

fn part1(program: Vec<isize>) {
    execute(program, 1);
}
fn part2(program: Vec<isize>) {
    execute(program, 5);
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let program : Vec<isize> = contents.split(",").map(|i| i.parse::<isize>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}