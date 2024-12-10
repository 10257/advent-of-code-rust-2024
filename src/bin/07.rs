use smallvec::{smallvec, SmallVec};

advent_of_code::solution!(7);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn recurse_calc(nb_list: &SmallVec<[u64; 15]>, pos: usize, acc: u64, to_calc: &u64) -> bool {
    if acc > *to_calc {
        return false;
    }
    if pos == nb_list.len() {
        if acc == *to_calc {
            return true;
        }
        return false;
    }
    recurse_calc(nb_list, pos + 1, acc + nb_list[pos], to_calc)
        || recurse_calc(nb_list, pos + 1, acc * nb_list[pos], to_calc)
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines().fold(0, |mut acc, line| {
        let mut line_part = line.split(':');
        let to_calc = line_part.next().unwrap().parse::<u64>().unwrap();
        let mut numbers: SmallVec<[u64; 15]> = smallvec![];
        for nb_str in line_part.next().unwrap().split_ascii_whitespace() {
            numbers.push(nb_str.parse().unwrap());
        }
        // println!("================\nlist:{:?}", numbers);
        if recurse_calc(&numbers, 1, numbers[0], &to_calc) {
            acc += to_calc;
        }
        acc
    });
    Some(result)
}

pub fn recurse_calc_part2(
    nb_list: &SmallVec<[u64; 15]>,
    pos: usize,
    acc: u64,
    to_calc: &u64,
) -> bool {
    if acc > *to_calc {
        return false;
    }
    if pos == nb_list.len() {
        if acc == *to_calc {
            return true;
        }
        return false;
    }

    recurse_calc_part2(nb_list, pos + 1, acc + nb_list[pos], to_calc)
        || recurse_calc_part2(nb_list, pos + 1, acc * nb_list[pos], to_calc)
        || recurse_calc_part2(
            nb_list,
            pos + 1,
            acc * 10u64.pow(nb_list[pos].ilog10() + 1) + nb_list[pos],
            to_calc,
        )
    // println!(
    //     "pos:{:?} -- acc:{:?} + nb:{:?} = {:?}",
    //     pos,
    //     acc,
    //     nb_list[pos],
    //     acc + nb_list[pos]
    // );
    // println!(
    //     "pos:{:?} -- acc:{:?} * nb:{:?} = {:?}",
    //     pos,
    //     acc,
    //     nb_list[pos],
    //     acc * nb_list[pos]
    // );
    // println!(
    //     "pos:{:?} -- acc:{:?} || nb:{:?} = {:?}",
    //     pos,
    //     acc,
    //     nb_list[pos],
    //     acc * 10u64.pow(nb_list[pos].ilog10() + 1) + nb_list[pos]
    // );
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines().fold(0, |mut acc, line| {
        let mut line_part = line.split(':');
        let to_calc = line_part.next().unwrap().parse::<u64>().unwrap();
        let mut numbers: SmallVec<[u64; 15]> = smallvec![];
        for nb_str in line_part.next().unwrap().split_ascii_whitespace() {
            numbers.push(nb_str.parse().unwrap());
        }
        if recurse_calc_part2(&numbers, 1, numbers[0], &to_calc) {
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
