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

#[derive(Debug)]
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

#[derive(Debug)]
struct JmpOutOfBoundsError;

#[derive(Debug)]
struct CPU {
    accumulator: isize,
    instruction_pointer: usize,
}

impl CPU {
    fn run_instruction(&mut self, instruction: &Instruction) -> isize {
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

    fn run_program_until_loop(&mut self, program: &Program) -> Result<(), JmpOutOfBoundsError> {
        let mut visited = vec![false; program.instructions.len()];

        while !visited[self.instruction_pointer] {
            visited[self.instruction_pointer] = true;
            let delta = self.run_instruction(&program.instructions[self.instruction_pointer]);
            self.update_instruction_pointer(delta)?;
        }

        Ok(())
    }
}

fn star_one(program: &Program) -> isize {
    let mut cpu = CPU {
        accumulator: 0,
        instruction_pointer: 0,
    };

    cpu.run_program_until_loop(program).expect("Program should not jump out of bounds");
    cpu.accumulator
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

        let mut cpu = super::CPU { accumulator: 0, instruction_pointer: 0 };
        cpu.run_program_until_loop(&program).expect("Program should not jump out of bounds");
        assert_eq!(cpu.accumulator, 5);
    }
}
