use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(5);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: FxHashMap<u32, FxHashSet<u32>> = FxHashMap::default();
    let (rules_str, to_print_str) = input.split("\n\n").next_tuple().unwrap();
    for line in rules_str.lines() {
        let (page_n1, page_n2) = line
            .split('|')
            .map(|nb| nb.parse::<u32>().unwrap())
            .next_tuple()
            .unwrap();
        rules.entry(page_n1).or_default().insert(page_n2);
    }
    println!("{:?}", rules);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
