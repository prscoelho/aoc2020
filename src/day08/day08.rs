use std::collections::HashSet;
#[derive(Clone, Copy)]
struct Instruction {
    value: i64,
    kind: InstructionKind,
}
#[derive(Clone, Copy)]
enum InstructionKind {
    ACC,
    JUMP,
    NOP,
}

fn parse_instruction(line: &str) -> Instruction {
    let mut tokens = line.split(" ");
    let instr = tokens.next().unwrap();
    let value = tokens.next().unwrap().parse::<i64>().unwrap();

    let kind = match instr {
        "jmp" => InstructionKind::JUMP,
        "nop" => InstructionKind::NOP,
        "acc" => InstructionKind::ACC,
        _ => panic!("Unexpected instruction"),
    };
    Instruction { value, kind }
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
        let idx = instruction_pointer as usize;
        let Instruction { value, kind } = memory[idx];
        match kind {
            InstructionKind::ACC => {
                acc += value;
            }
            InstructionKind::JUMP => {
                instruction_pointer += value;
                continue;
            }
            InstructionKind::NOP => {}
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
        let new_kind = match instr.kind {
            InstructionKind::JUMP => InstructionKind::NOP,
            InstructionKind::NOP => InstructionKind::JUMP,
            InstructionKind::ACC => {
                continue;
            }
        };
        let mut new_memory = memory.clone();
        new_memory[idx].kind = new_kind;

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
