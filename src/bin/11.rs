use itertools::Itertools;

advent_of_code::solution!(11);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn create_stones<const N: usize>(input: &str) -> Option<u32> {
    let mut stones: Vec<u64> = input.split_ascii_whitespace().map(|stone| stone.parse().unwrap()).collect_vec();
    let mut new_stones: Vec<u64> = vec![];

    for _ in 0..N {
        stones.iter().for_each(|&stone| {
            if stone == 0 {
                new_stones.push(1);
                return;
            }
            let len = stone.ilog10() + 1;
            match len % 2 {
                0 => {
                    new_stones.push(stone.div_euclid(10u64.pow(len/2)));
                    new_stones.push(stone.rem_euclid(10u64.pow(len/2)));
                },
                1 => new_stones.push(stone * 2024),
                _ => {}
            }
        });
        stones = new_stones.clone();
        new_stones.clear();
    }
    Some(stones.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    create_stones::<25>(input)
}

pub fn create_stone_alt(stones_src: &mut [u64]) -> Option<u32> {
    let mut stones: Vec<u64> = vec![1];
    stones.clone_from_slice(stones_src);
    let mut new_stones: Vec<u64> = vec![];
    for _ in 0..75 {
        stones.iter().for_each(|&stone| {
            if stone == 0 {
                new_stones.push(1);
                return;
            }
            let len = stone.ilog10() + 1;
            match len % 2 {
                0 => {
                    new_stones.push(stone.div_euclid(10u64.pow(len/2)));
                    new_stones.push(stone.rem_euclid(10u64.pow(len/2)));
                },
                1 => new_stones.push(stone * 2024),
                _ => {}
            }
        });
        stones = new_stones.clone();
        new_stones.clear();
    }
    Some(stones.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let stones: Vec<u64> = input.split_ascii_whitespace().map(|stone| stone.parse().unwrap()).collect_vec();
    // let mut new_stones: Vec<u64> = vec![];

    let sum_stones: Option<u32> = stones.iter().map(|&stone| {
        let mut stones = vec![stone];
        create_stone_alt(&mut stones)
    }).sum();
    sum_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
