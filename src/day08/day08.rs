use std::collections::HashSet;
#[derive(Clone, Copy)]
enum Instruction {
    ACC(i64),
    JUMP(i64),
    NOP(i64),
}

fn parse_instruction(line: &str) -> Instruction {
    let mut tokens = line.split(" ");
    let instr = tokens.next().unwrap();
    let val = tokens.next().unwrap().parse::<i64>().unwrap();

    match instr {
        "jmp" => Instruction::JUMP(val),
        "nop" => Instruction::NOP(val),
        "acc" => Instruction::ACC(val),
        _ => panic!("Unexpected instruction"),
    }
}

fn read_memory(input: &str) -> Vec<Instruction> {
    let mut memory = Vec::new();

    for line in input.lines() {
        memory.push(parse_instruction(line));
    }
    memory
}

// returns acc, completed
fn run(memory: Vec<Instruction>) -> (i64, bool) {
    let mut seen = HashSet::new();
    let mut instruction_pointer: i64 = 0;
    let mut acc = 0;
    while (instruction_pointer as usize) < memory.len() {
        if !seen.insert(instruction_pointer) {
            return (acc, false);
        }
        match memory[instruction_pointer as usize] {
            Instruction::ACC(val) => {
                acc += val;
            }
            Instruction::JUMP(val) => {
                instruction_pointer += val;
                continue;
            }
            Instruction::NOP(_) => {}
        }
        instruction_pointer += 1;
    }
    (acc, true)
}

pub fn part1(input: &str) -> i64 {
    let memory = read_memory(input);
    run(memory).0
}

pub fn part2(input: &str) -> i64 {
    let memory = read_memory(input);

    for (idx, instr) in memory.iter().enumerate() {
        let new_instruction = match instr {
            Instruction::JUMP(val) => Instruction::NOP(*val),
            Instruction::NOP(val) => Instruction::JUMP(*val),
            Instruction::ACC(_) => {
                continue;
            }
        };
        let mut new_memory = memory.clone();
        new_memory[idx] = new_instruction;

        let (acc, completed) = run(new_memory);
        if completed {
            return acc;
        }
    }
    unreachable!()
}

mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 1749);
    }
    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 515);
    }
}
