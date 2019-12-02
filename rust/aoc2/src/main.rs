use std::fs;

fn execute(mut input: Vec<usize>) -> usize {
    let mut i = 0;
    loop {
        match input[i] {
            1 => {
                let position = input[i+3];
                input[position] = input[input[i+2]] + input[input[i+1]]
            },
            2 => {
                let position = input[i+3];
                input[position] = input[input[i+2]] * input[input[i+1]]
            },
            99 => {
                break
            },
            _ => {}
        }
        i += 4;
    }
    input[0]
}

fn part1(mut input: Vec<usize>) -> usize {
    input[1] = 12;
    input[2] = 2;
    execute(input)
}

fn part2(mut input: Vec<usize>) -> usize {
    for a in 0..99{
        for b in 0..99{
            input[1] = a;
            input[2] = b;
            let result = execute(input.clone());
            if result == 19690720 {
                return 100 * a + b;
            }
        }
    }
    panic!("Could not solve part 2")
}

fn main() {
    let contents = fs::read_to_string("day2.txt")
        .expect("Something went wrong reading the file");
    let input : Vec<usize> = contents.split(",").map(|i| i.parse::<usize>().unwrap()).collect();
    println!("Content: {:?}", &input[0..5]);
    
    println!("Part 1 result: {}", part1(input.clone()));
    println!("Part 2 result: {}", part2(input.clone()));
}