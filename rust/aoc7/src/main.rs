use std::fs;
use std::collections::VecDeque;
use permute::permutations_of;

fn get_modes(modes_num: usize) -> Vec<bool> {
    (0..3).map(|i| {(modes_num / 10_usize.pow(i) % 10_usize.pow(i+1)) == 1}).collect()
}

fn execute(mut program: Vec<isize>, mut i: usize, mut input: VecDeque<isize>) -> ((Vec<isize>, usize, VecDeque<isize>), Option<isize>) {
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
                program[position] = input.pop_front().unwrap();
                i += 2;
            }
            4 => {
                let op1 = if modes[0] {program[i+1]} else {program[program[i+1] as usize]};
                i += 2;
                return ((program, i, input), Some(op1));
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
                return ((program, i, input), None);
            }
            _ => { panic!() }
        }
    }
}

fn part1(program: Vec<isize>) -> isize {
    let phases: Vec<isize> = vec![0,1,2,3,4];
    permutations_of(&phases)
        .map(|setting| {
            let mut stage_var = 0;
            for i in setting {
                let input = VecDeque::from(vec![*i, stage_var]);
                let result = execute(program.clone(), 0, input);
                stage_var = result.1.unwrap()
            }
            stage_var
        }).max().unwrap_or(0)
}
fn part2(program: Vec<isize>) -> isize {
    let phases: Vec<isize> = vec![5,6,7,8,9];
    permutations_of(&phases)
        .map(|setting| {
            let mut states: Vec<(Vec<isize>, usize, VecDeque<isize>)> = setting.map(|phase| (program.clone(), 0 as usize, VecDeque::from(vec![*phase]))).collect();
            let (mut i, mut output) = (0, 0);
            loop {
                states[i%5].2.push_back(output);
                let result = execute(states[i%5].0.clone(), states[i%5].1, states[i%5].2.clone());
                if result.1 == None {
                    return output;
                }
                states[i%5] = result.0.clone();
                output = result.1.unwrap();
                i += 1;
            }
        }).max().unwrap_or(0)
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Could not read file");
    let program : Vec<isize> = contents.split(",").map(|i| i.parse::<isize>().unwrap()).collect();
    println!("{}", part1(program.clone()));
    println!("{}", part2(program.clone()));
}