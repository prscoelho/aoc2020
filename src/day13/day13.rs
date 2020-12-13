fn parse_values(input: &str) -> (i64, Vec<(i64, i64)>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let mut busses = Vec::new();

    for (offset, elements) in lines.next().unwrap().split(",").enumerate() {
        if let Ok(bus_id) = elements.parse::<i64>() {
            busses.push((offset as i64, bus_id));
        }
    }

    (timestamp, busses)
}

pub fn part1(input: &str) -> i64 {
    let (timestamp, busses) = parse_values(input);
    let (index, best) = busses
        .into_iter()
        // timestamp % bus = how much time has passed since bus last departed
        // for example, timestamp = 8, bus = 7 => 8 % 7 = 1; bus departed t = 1 ago
        // bus - (timestamp % bus) = how much time to wait until bus departs
        // in the example above, 7 - 1 = 6; waiting 6 for bus to arrive again
        .map(|(_, bus)| (bus, bus - (timestamp % bus)))
        .min_by_key(|(_, wait_time)| *wait_time)
        .unwrap();

    index * best
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part2(input: &str) -> i64 {
    let (_, busses) = parse_values(input);

    let (offsets, ids): (Vec<i64>, Vec<i64>) = busses.iter().cloned().unzip();

    ids.iter().product::<i64>() - chinese_remainder(&offsets, &ids).unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 153);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 471793476184394);
    }

    #[test]
    fn part2_example1() {
        let input = include_str!("example1");
        assert_eq!(super::part2(input), 1068781);
    }

    #[test]
    fn part2_example2() {
        let input = include_str!("example2");
        assert_eq!(super::part2(input), 3417);
    }
}
