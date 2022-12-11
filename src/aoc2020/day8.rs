use std::collections::HashSet;

/*
--- Day 8: Handheld Halting ---

Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those
drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.

Their handheld game console won't turn on! They ask if you can take a look.

You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.

The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).

acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0.
After an acc instruction, the instruction immediately below it is executed next.
jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction,
jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
For example, consider the following program:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
These instructions are visited in this order:

nop +0  | 1
acc +1  | 2, 8(!)
jmp +4  | 3
acc +3  | 6
jmp -3  | 7
acc -99 |
acc +1  | 4
jmp -4  | 5
acc +6  |
First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator
from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.

This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.

Immediately before the program would run an instruction a second time, the value in the accumulator is 5.

Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
1818

--- Part Two ---

After some careful analysis, you believe that exactly one instruction is corrupted.

Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)

The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot
code and make it terminate correctly.

For example, consider the same program from above:

nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp
instructions, the program will still eventually find another jmp instruction and loop forever.

However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:

nop +0  | 1
acc +1  | 2
jmp +4  | 3
acc +3  |
jmp -3  |
acc -99 |
acc +1  | 4
nop -4  | 5
acc +6  | 6
After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates,
the accumulator contains the value 8 (acc +1, acc +1, acc +6).

Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?

631
*/
#[derive(Debug, Clone)]
pub enum Instruction {
    Acc(isize),
    Nop(isize),
    Jmp(isize),
}

struct ExecutionReport {
    result: isize,
    terminated: bool,
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    let input_lines = input.lines();
    let mut instructions: Vec<Instruction> =
        Vec::with_capacity(input_lines.size_hint().1.unwrap_or(0));

    input_lines.for_each(|line| {
        let mut split_line = line.split_ascii_whitespace();
        let op_code = split_line.next().unwrap();
        let number_code = split_line.next().unwrap().split_at(1);
        let number = match number_code.0 {
            "+" => number_code.1.parse::<isize>().unwrap(),
            "-" => -(number_code.1.parse::<isize>().unwrap()),
            _ => panic!(
                "unknow sign for number code: {}{}",
                number_code.0, number_code.1
            ),
        };
        let instruction = match op_code {
            "acc" => Instruction::Acc(number),
            "nop" => Instruction::Nop(number),
            "jmp" => Instruction::Jmp(number),
            _ => panic!("encountered unecppected OpCode: {}", op_code),
        };
        instructions.push(instruction);
    });
    instructions
}

pub fn part1(instructions: &[Instruction]) -> isize {
    execute_boot_code(instructions).result
}

fn execute_boot_code(instructions: &[Instruction]) -> ExecutionReport {
    let mut acced = 0;
    let mut executed_op_index: HashSet<isize> = HashSet::new();
    let mut op_index = 0;
    let mut terminated = false;
    while !terminated && !executed_op_index.contains(&op_index) {
        executed_op_index.insert(op_index);
        match instructions.get(op_index as usize) {
            Some(instruction) => match instruction {
                Instruction::Acc(n) => {
                    acced += n;
                    op_index += 1
                }
                Instruction::Jmp(j) => op_index += j,
                Instruction::Nop(_) => op_index += 1,
            },
            None => terminated = true,
        }
    }
    ExecutionReport {
        result: acced,
        terminated,
    }
}

pub fn part2(instructions: &[Instruction]) -> isize {
    //run unaltered
    let mut execution_report = execute_boot_code(instructions);
    if execution_report.terminated {
        return execution_report.result;
    }
    // replace jmps with nops (one each time)
    execution_report = fix_instructions(instructions, Instruction::Nop(0), &Instruction::Jmp);
    if execution_report.terminated {
        execution_report.result
    } else {
        //replace nops with jmps (one each time)
        execution_report = fix_instructions(instructions, Instruction::Jmp(0), &Instruction::Nop);
        if execution_report.terminated {
            execution_report.result
        } else {
            panic!("instructions unfixable!")
        }
    }
}

fn fix_instructions(
    instructions: &[Instruction],
    old_op_code: Instruction,
    new_op_code: &dyn Fn(isize) -> Instruction,
) -> ExecutionReport {
    let mut execution_report = ExecutionReport {
        result: -1,
        terminated: false,
    };
    for (index, instruction) in instructions.iter().enumerate() {
        match old_op_code {
            Instruction::Jmp(_) => match instruction {
                Instruction::Jmp(op_number) => {
                    execution_report =
                        try_replace_op_code_and_exec(index, instructions, new_op_code, op_number);
                    if execution_report.terminated {
                        return execution_report;
                    }
                }
                _ => continue,
            },
            Instruction::Nop(_) => match instruction {
                Instruction::Nop(op_number) => {
                    execution_report =
                        try_replace_op_code_and_exec(index, instructions, new_op_code, op_number);
                    if execution_report.terminated {
                        return execution_report;
                    }
                }
                _ => continue,
            },
            Instruction::Acc(_) => {
                panic!("Instruction 'Acc' is not supported for replacement!")
            }
        }
    }
    execution_report
}

fn try_replace_op_code_and_exec(
    replace_at_index: usize,
    instructions: &[Instruction],
    new_op_code: &dyn Fn(isize) -> Instruction,
    op_number: &isize,
) -> ExecutionReport {
    let mut changed_instructions = instructions.to_vec();
    changed_instructions[replace_at_index] = new_op_code(*op_number);
    execute_boot_code(&changed_instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_PART1: &str = "nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6";

    fn parse_test_input() -> Vec<Instruction> {
        parse_input(SAMPLE_INPUT_PART1)
    }

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(&parse_test_input()))
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(&parse_test_input()))
    }
}
