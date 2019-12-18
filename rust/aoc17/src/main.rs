use std::fs;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::char;
use std::str;
use pcre2::bytes::Regex;

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
    fn add(self, point: Point) -> Point {
        return Point::new(self.x + point.x, self.y + point.y);
    }
    fn alignment(self) -> i64 {
        return self.x * self.y;
    }
}

fn print_grid(grid: HashMap<Point,char>) {
    let min_x: i64 = grid.iter().min_by_key(|(pt, _)| (*pt).x).unwrap().0.x;
    let max_x: i64 = grid.iter().max_by_key(|(pt, _)| (*pt).x).unwrap().0.x;
    let min_y: i64 = grid.iter().min_by_key(|(pt, _)| (*pt).y).unwrap().0.y;
    let max_y: i64 = grid.iter().max_by_key(|(pt, _)| (*pt).y).unwrap().0.y;
    for y in (min_y..max_y+1).rev() {
        for x in min_x..max_x+1 {
            print!("{}", grid.get(&Point::new(x,y)).unwrap());
        }
        println!("");
    }
}

fn align_params(mut grid: HashMap<Point,char>) {
    let min_x: i64 = grid.iter().min_by_key(|(pt, _)| (*pt).x).unwrap().0.x;
    let max_x: i64 = grid.iter().max_by_key(|(pt, _)| (*pt).x).unwrap().0.x;
    let min_y: i64 = grid.iter().min_by_key(|(pt, _)| (*pt).y).unwrap().0.y;
    let max_y: i64 = grid.iter().max_by_key(|(pt, _)| (*pt).y).unwrap().0.y;
    for y in min_y+1..max_y {
        for x in min_x+1..max_x {
            if *(grid.get(&Point::new(x,y)).unwrap()) == '#'
            && *(grid.get(&Point::new(x,y+1)).unwrap()) == '#'
            && *(grid.get(&Point::new(x,y-1)).unwrap()) == '#'
            && *(grid.get(&Point::new(x+1,y)).unwrap()) == '#'
            && *(grid.get(&Point::new(x-1,y)).unwrap()) == '#' {
                grid.insert(Point::new(x, y), 'O');
            }
        }
    }
    let intersections: HashMap<Point,char> = grid.into_iter().filter(|(_, c)| *c == 'O').collect();
    let alignment_sum: i64 = intersections.iter().map(|(pt, _)| (*pt).alignment()).sum();
    println!("sum is {}", alignment_sum);
}

fn get_grid(programvec: Vec<i64>) -> HashMap<Point,char> {
    let program: BTreeMap<usize, i64> = programvec.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut grid: HashMap<Point,char> = HashMap::new();
    let mut computer = Computer::new(program);
    let (mut column, mut row) = (0, 0);
    loop {
        let result = computer.execute(&mut VecDeque::<i64>::new());
        if result == None {
            break;
        }
        let tile = char::from_u32(result.unwrap() as u32).unwrap();
        if tile == '\n' {
            row += 1;
            column = 0;
        }
        else {
            grid.insert(Point::new(column, row), tile);
            column += 1;
        }
    }
    let max_y: i64 = grid.iter().max_by_key(|(pt, _)| (*pt).y).unwrap().0.y;
    grid = grid.iter().map(|(pt, c)| (Point::new(pt.x, max_y - pt.y), *c)).collect();
    return grid;
}

fn part1(programvec: Vec<i64>) {
    let grid = get_grid(programvec);    
    print_grid(grid.clone());
    align_params(grid.clone());
}
    
fn part2(programvec: Vec<i64>) {
    let grid = get_grid(programvec.clone());
    let directions = vec![Point::new(0,1), Point::new(1,0), Point::new(0,-1), Point::new(-1,0)];
    let ship_directions = ['^', '>', 'v', '<'];
    let ship_char: char = *(grid.iter().find(|(_, &c)| c == '^' || c == '>' || c == 'v' || c == '<').unwrap().1);
    let mut position: Point = *(grid.iter().find(|(_, &c)| c == '^').unwrap().0);
    let mut facing = directions[ship_directions.iter().position(|&d| d == ship_char).unwrap()];
    let mut next_dir: usize = direction_shift(grid.clone(), position, facing).unwrap();
    let mut segments: String = "".to_string();
    let mut cumulator = 0;
    loop {
        match next_dir {
            0 => {
                cumulator += 1;
                position = position.add(facing);
            },
            1 => { 
                if cumulator != 0 {
                    segments += &format!("{},", cumulator);
                }
                segments += "R,";
                cumulator = 0;
                facing = directions[(directions.iter().position(|&d| d == facing).unwrap()+1)%4];
            },
            3 => { 
                if cumulator != 0 {
                    segments += &format!("{},", cumulator);
                }
                segments += "L,";
                cumulator = 0;
                facing = directions[(directions.iter().position(|&d| d == facing).unwrap()+3)%4];
            },
            _ => { 
                if cumulator != 0 {
                    segments += &format!("{},", cumulator);
                }
                break;
            }
        }
        next_dir = direction_shift(grid.clone(), position, facing).unwrap();
    }
    println!("{}",segments);

    let re = Regex::new(r"^(.{1,21})\1*(.{1,21})(?:\1|\2)*(.{1,21})(?:\1|\2|\3)*$").unwrap();
    let caps = re.captures(segments.as_bytes()).unwrap().unwrap();
    let group_a = str::from_utf8(caps.get(1).map_or(&b""[..], |m| m.as_bytes())).unwrap();
    let group_b = str::from_utf8(caps.get(2).map_or(&b""[..], |m| m.as_bytes())).unwrap();
    let group_c = str::from_utf8(caps.get(3).map_or(&b""[..], |m| m.as_bytes())).unwrap();
    let trim_group_a = &(group_a.to_string()[0..group_a.len()-1]);
    let trim_group_b = &(group_b.to_string()[0..group_b.len()-1]);
    let trim_group_c = &(group_c.to_string()[0..group_c.len()-1]);
    let main_routine = segments.replace(trim_group_a, "A").replace(trim_group_b, "B").replace(trim_group_c, "C");
    let trim_routine = &(main_routine.to_string()[0..main_routine.len()-1]);
    let routine = format!("{}\n{}\n{}\n{}\nn\n", trim_routine, trim_group_a, trim_group_b, trim_group_c);
    println!("routine is {}", routine);
    let mut input: VecDeque<i64> = routine.chars().map(|c| c as i64).collect();
    let mut program = programvec.clone();
    program[0] = 2;
    let programmap: BTreeMap<usize, i64> = program.iter().enumerate().map(|(i, x)| (i,*x)).collect();
    let mut computer = Computer::new(programmap);
    loop {
        let result = computer.execute(&mut input);
        if result == None {
            break;
        }
        if result.unwrap() > 128 {
            println!("SCORE IS {}", result.unwrap());
        }
        else {
            print!("{}", char::from_u32(result.unwrap() as u32).unwrap());
        }
    }
}

fn direction_shift(grid: HashMap<Point,char>, position: Point, facing: Point) -> Option<usize> {
    let directions = vec![Point::new(0,1), Point::new(1,0), Point::new(0,-1), Point::new(-1,0)];
    let clockwise_alt = directions[(directions.iter().position(|&d| d == facing).unwrap()+1)%4];
    let uturn = directions[(directions.iter().position(|&d| d == facing).unwrap()+2)%4];
    let cclockwise_alt = directions[(directions.iter().position(|&d| d == facing).unwrap()+3)%4];
    if grid.get(&position.add(facing)) != None && *grid.get(&position.add(facing)).unwrap() == '#' {
        return Some(0);
    }
    else if grid.get(&position.add(clockwise_alt)) != None && *grid.get(&position.add(clockwise_alt)).unwrap() == '#' {
        return Some(1);
    }
    else if grid.get(&position.add(cclockwise_alt)) != None && *grid.get(&position.add(cclockwise_alt)).unwrap() == '#' {
        return Some(3);
    }
    else if grid.get(&position.add(uturn)) != None && *grid.get(&position.add(uturn)).unwrap() == '#' {
        return Some(2);
    }
    panic!("Somwhow can't find any scaffold around {:?} facing {:?}", position, facing);
}

fn main() {
    let contents = fs::read_to_string("day17.txt").expect("Could not read file");
    let program : Vec<i64> = contents.split(",").map(|i| i.parse::<i64>().unwrap()).collect();
    part1(program.clone());
    part2(program.clone());
}