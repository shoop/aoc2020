use core::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct IncorrectInstruction;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    operation: Operation,
    operand: isize,
}

impl Instruction {
    fn from_line(line: &str) -> Result<Self, IncorrectInstruction> {
        let mut split = line.split(" ");
        let raw_operation = split.next().unwrap();
        let raw_operand = split.next().unwrap();
        if split.next() != None {
            return Err(IncorrectInstruction);
        }

        let operation = match raw_operation {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => return Err(IncorrectInstruction),
        };

        if let Ok(operand) = raw_operand.parse::<isize>() {
            Ok(Instruction { operation: operation, operand: operand })
        } else {
            Err(IncorrectInstruction)
        }
    }
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn add_instruction(&mut self, line: &str) -> Result<(), IncorrectInstruction> {
        self.instructions.push(Instruction::from_line(&line)?);
        Ok(())
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        Program { instructions: self.instructions.clone() }
    }
}

#[derive(Debug)]
struct JmpOutOfBoundsError;

#[derive(Debug)]
struct CPU<'a> {
    accumulator: isize,
    instruction_pointer: usize,
    program: &'a Program,
    visited: Vec<bool>,
}

impl<'a> Clone for CPU<'a> {
    fn clone(&self) -> CPU<'a> {
        CPU { accumulator: self.accumulator, instruction_pointer: self.instruction_pointer, program: self.program, visited: self.visited.clone() }
    }
}

impl<'a> CPU<'_> {
    fn new(program: &'a Program) -> CPU<'a> {
        CPU { accumulator: 0, instruction_pointer: 0, program: program, visited: vec![false; program.instructions.len()] }
    }

    fn run_instruction(&mut self, instruction: &Instruction) -> isize {
        //println!("Running IP {} instruction {:?} {:+04} acc {}", self.instruction_pointer, instruction.operation, instruction.operand, self.accumulator);
        match instruction.operation {
            Operation::Nop => 1,
            Operation::Acc => {
                self.accumulator += instruction.operand;
                1
            }
            Operation::Jmp => instruction.operand,
        }
    }

    fn update_instruction_pointer(&mut self, delta: isize) -> Result<(), JmpOutOfBoundsError> {
        type IpModifier = fn(usize, usize) -> Option<usize>;
        let mut f: IpModifier = usize::checked_add;
        if delta < 0 {
            f = usize::checked_sub;
        }

        match f(self.instruction_pointer, delta.abs() as usize) {
            Some(ip) => self.instruction_pointer = ip,
            None => return Err(JmpOutOfBoundsError),
        }

        Ok(())
    }

    fn run_program_until_loop(&mut self) -> Result<bool, JmpOutOfBoundsError> {
        while !self.visited[self.instruction_pointer] {
            self.visited[self.instruction_pointer] = true;
            let delta = self.run_instruction(&self.program.instructions[self.instruction_pointer]);
            self.update_instruction_pointer(delta)?;

            if self.instruction_pointer >= self.program.instructions.len() {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

fn star_one(program: &Program) -> isize {
    let mut cpu = CPU::new(program);
    cpu.run_program_until_loop().expect("Program should not jump out of bounds");
    cpu.accumulator
}

fn star_two(program: &Program) -> isize {
    let mut instructions_ran: usize = 0;
    let mut modded = program.clone();
    let mut modded_ip = 0;
    loop {
        let mut cpu = CPU::new(&modded);
        let done = cpu.run_program_until_loop().expect("Program should not jump out of bounds");
        instructions_ran += cpu.visited.into_iter().filter(|x| *x).count();
        if done {
            println!("Ran {} instructions", instructions_ran);
            return cpu.accumulator
        }

        // Mod next instruction in sequence
        // TODO: figure out smart walk
        for (index, instruction) in program.instructions.iter().enumerate().skip(modded_ip) {
            if instruction.operation == Operation::Jmp || instruction.operation == Operation::Nop {
                modded_ip = index;
                break;
            }
        }

        modded = program.clone();
        match modded.instructions[modded_ip].operation {
            Operation::Acc => {},
            Operation::Jmp => modded.instructions[modded_ip].operation = Operation::Nop,
            Operation::Nop => modded.instructions[modded_ip].operation = Operation::Jmp,
        }
        modded_ip += 1;
    }
}

fn main() {
    let file = File::open("./input").expect("Unreadable input file ./input");
    let mut program = Program { instructions: vec![] };
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(line) => program.add_instruction(&line).expect("Invalid data in input file"),
            Err(_) => panic!("Could not read line"),
        }
    }

    println!("Star 1:");
    let acc_value_before_loop = star_one(&program);
    println!("Accumulator value before the first loop: {}", acc_value_before_loop);

    println!("Star 2:");
    let acc_value_after_exit = star_two(&program);
    println!("Accumulator value after exit modification: {}", acc_value_after_exit);
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_star_one() {
        let mut program = super::Program { instructions: vec![] };
        for line in TEST_DATA.lines().map(|x| x.to_string()) {
            program.add_instruction(&line).expect("Invalid data in input file");
        }

        assert_eq!(program.instructions.len(), 9);
        assert_eq!(program.instructions[0].operation, super::Operation::Nop);
        assert_eq!(program.instructions[1].operation, super::Operation::Acc);
        assert_eq!(program.instructions[1].operand, 1);

        let mut cpu = super::CPU::new(&program);
        cpu.run_program_until_loop().expect("Program should not jump out of bounds");
        assert_eq!(cpu.accumulator, 5);
    }

    #[test]
    fn test_star_two() {
        let mut program = super::Program { instructions: vec![] };
        for line in TEST_DATA.lines().map(|x| x.to_string()) {
            program.add_instruction(&line).expect("Invalid data in input file");
        }

        assert_eq!(program.instructions.len(), 9);
        assert_eq!(program.instructions[0].operation, super::Operation::Nop);
        assert_eq!(program.instructions[1].operation, super::Operation::Acc);
        assert_eq!(program.instructions[1].operand, 1);

        let result = super::star_two(&program);
        assert_eq!(result, 8);
    }
}
