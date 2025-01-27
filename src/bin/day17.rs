use regex::Regex;
use std::fs;

struct Computer {
    a: i32,
    b: i32,
    c: i32,
    ip: usize,
    program: Vec<u8>,
    output: Vec<i32>,
}

impl Computer {
    fn new(a: i32, b: i32, c: i32, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn execute_instruction(&mut self, code: u8, operand: u8) {
        match code {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(operand),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => panic!("Unknown instruction code {}", code),
        }
    }

    fn get_operand(&mut self, operand: u8) -> i32 {
        match operand {
            0..=3 => operand as i32,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unknown operand code {}", operand),
        }
    }

    fn adv(&mut self, operand: u8) {
        let combo = self.get_operand(operand);
        self.a = self.a / 2_i32.pow(combo as u32);
        self.ip += 2;
    }

    fn bxl(&mut self, operand: u8) {
        self.b ^= operand as i32;
        self.ip += 2;
    }

    fn bst(&mut self, operand: u8) {
        let combo = self.get_operand(operand);
        self.b = combo % 8;
        self.ip += 2;
    }

    fn jnz(&mut self, operand: u8) {
        if self.a != 0 {
            self.ip = operand as usize;
        } else {
            self.ip += 2;
        }
    }

    fn bxc(&mut self, _operand: u8) {
        self.b ^= self.c;
        self.ip += 2;
    }

    fn out(&mut self, operand: u8) {
        let combo = self.get_operand(operand);
        self.output.push(combo % 8);
        self.ip += 2;
    }

    fn bdv(&mut self, operand: u8) {
        let combo = self.get_operand(operand);
        self.b = self.a / 2_i32.pow(combo as u32);
        self.ip += 2;
    }

    fn cdv(&mut self, operand: u8) {
        let combo = self.get_operand(operand);
        self.c = self.a / 2_i32.pow(combo as u32);
        self.ip += 2;
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            let code = self.program.get(self.ip).cloned().unwrap();
            let operand = self.program.get(self.ip + 1).cloned().unwrap();
            self.execute_instruction(code, operand);
        }
    }

    fn get_output(&mut self) -> Vec<i32> {
        self.output.clone()
    }
}

#[derive(Debug)]
struct Input {
    a: i32,
    b: i32,
    c: i32,
    program: Vec<u8>,
}

fn read_input(path: &str) -> Input {
    let content = fs::read_to_string(path).unwrap();
    let re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<i32> = re
        .find_iter(&content)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect();
    let program: Vec<u8> = numbers[3..].iter().map(|&x| x as u8).collect();
    Input {
        a: numbers[0],
        b: numbers[1],
        c: numbers[2],
        program,
    }
}

fn main() {
    let data = read_input("data/day17.txt");
    let mut computer = Computer::new(data.a, data.b, data.c, data.program);
    computer.run();
    println!(
        "Part 1: {:?}",
        computer
            .get_output()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    );
}

#[cfg(test)]
mod tests {
    use crate::Computer;

    #[test]
    fn test_part1() {
        let mut computer = Computer::new(729, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run();
        assert_eq!(computer.get_output(), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }
}
