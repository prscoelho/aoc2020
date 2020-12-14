use std::collections::HashMap;

enum Instruction {
    Mask(u64, u64),   // Mask(xs, values)
    Update(u64, u64), // Update(location, value)
}

fn read_mask(mask_str: &str) -> (u64, u64) {
    let mut xs = 0;
    let mut values = 0;
    for c in mask_str.chars() {
        xs <<= 1;
        values <<= 1;
        match c {
            '1' => {
                values += 1;
            }
            '0' => {}
            'X' => {
                xs += 1;
            }
            _ => {
                unreachable!();
            }
        };
    }
    (xs, values)
}

fn read_input(input: &str) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            let (xs, values) = read_mask(&line[7..]);
            instructions.push(Instruction::Mask(xs, values));
        } else {
            let mut tokens_it = line.split(" = ");
            let left = tokens_it.next().unwrap();
            let find_num_end = left.find("]").unwrap();
            let memory_location = left[4..find_num_end].parse().unwrap();

            let right = tokens_it.next().unwrap();
            let value = right.parse().unwrap();

            instructions.push(Instruction::Update(memory_location, value));
        }
    }
    instructions
}

pub fn part1(input: &str) -> u64 {
    let instructions = read_input(input);
    let mut xs = 0;
    let mut values = 0;
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(x, v) => {
                xs = x;
                values = v;
            }
            Instruction::Update(location, value) => {
                let e = memory.entry(location).or_default();
                *e = (value & xs) | values;
            }
        }
    }
    memory.values().sum()
}

fn get_all_floating(mut location: u64, mut xs: u64) -> Vec<u64> {
    location = location & !xs; // set every x to 0 first
    let mut floating_bits = Vec::new();
    let mut bit_index = 0;
    while xs > 0 {
        if xs & 1 == 1 {
            floating_bits.push(bit_index);
        }
        bit_index += 1;
        xs >>= 1;
    }

    let mut result = Vec::new();
    result.push(location); // all floating bits as 0
    for floating in floating_bits {
        let bit = 1 << floating;

        for memory_location in result.clone().into_iter() {
            result.push(memory_location | bit);
        }
    }

    result
}

pub fn part2(input: &str) -> u64 {
    let instructions = read_input(input);
    let mut xs = 0;
    let mut values = 0;
    let mut memory = HashMap::new();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(x, v) => {
                xs = x;
                values = v;
            }
            Instruction::Update(location, value) => {
                let unfloating = location | values;
                for x in get_all_floating(unfloating, xs) {
                    let e = memory.entry(x).or_default();
                    *e = value;
                }
            }
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 17028179706934);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 3683236147222);
    }
}
