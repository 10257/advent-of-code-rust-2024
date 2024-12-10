use smallvec::{smallvec, SmallVec};

advent_of_code::solution!(7);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

type Operation = fn (u64, u64) -> u64;

pub fn add(a:u64, b:u64) -> u64 {
    a + b
}

pub fn mul(a:u64, b:u64) -> u64 {
    a * b
}

pub fn concat(a:u64, b:u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

pub fn recurse_calc<const N: usize>(nb_list: &[u64], pos: usize, acc: u64, to_calc: &u64, ops: &[Operation; N]) -> bool {
    if acc > *to_calc {
        return false;
    }
    if pos == nb_list.len() {
        if acc == *to_calc {
            return true;
        }
        return false;
    }
    // this is more the rust way!
    ops.iter().any(|op| {
        recurse_calc(nb_list, pos + 1, op(acc, nb_list[pos]), to_calc, ops)
    })
    // but this gives better timings!
    // if recurse_calc(nb_list, pos + 1, add(acc, nb_list[pos]), to_calc, ops)
    //     || recurse_calc(nb_list, pos + 1, mul(acc, nb_list[pos]), to_calc, ops) {
    //         return true
    //     }
    // N == 3 && recurse_calc(nb_list, pos + 1, concat(acc, nb_list[pos]), to_calc, ops)
}

pub fn part_one(input: &str) -> Option<u64> {
    let ops: [Operation; 2] = [add, mul];
    let result = input.lines().fold(0, |mut acc, line| {
        let mut line_part = line.split(':');
        let to_calc = line_part.next().unwrap().parse::<u64>().unwrap();
        let mut numbers: SmallVec<[u64; 15]> = smallvec![];
        for nb_str in line_part.next().unwrap().split_ascii_whitespace() {
            numbers.push(nb_str.parse().unwrap());
        }
        // println!("================\nlist:{:?}", numbers);
        if recurse_calc(&numbers, 1, numbers[0], &to_calc, &ops) {
            acc += to_calc;
        }
        acc
    });
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ops: [Operation; 3] = [add, mul, concat];
    let result = input.lines().fold(0, |mut acc, line| {
        let mut line_part = line.split(':');
        let to_calc = line_part.next().unwrap().parse::<u64>().unwrap();
        let mut numbers: SmallVec<[u64; 15]> = smallvec![];
        for nb_str in line_part.next().unwrap().split_ascii_whitespace() {
            numbers.push(nb_str.parse().unwrap());
        }
        // println!("================\nlist:{:?}", numbers);
        if recurse_calc(&numbers, 1, numbers[0], &to_calc, &ops) {
            acc += to_calc;
        }
        acc
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
