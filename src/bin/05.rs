use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{smallvec, SmallVec};

advent_of_code::solution!(5);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn check_page_order(
    rules: &FxHashMap<u8, FxHashSet<u8>>,
    page_list: &[u8],
    page_list_hash: &FxHashSet<u8>,
) -> bool {
    let mut intersect = 100;
    for &page_nb in page_list {
        let new_intersect = rules
            .get(&page_nb)
            .unwrap_or(&FxHashSet::default())
            .intersection(page_list_hash)
            .count() as u8;
        intersect = if new_intersect < intersect {
            new_intersect
        } else {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rules: FxHashMap<u8, FxHashSet<u8>> = FxHashMap::default();
    let (rules_str, to_print_str) = input.split("\n\n").next_tuple().unwrap();
    for line in rules_str.lines() {
        let (page_n1, page_n2) = line
            .split('|')
            .map(|nb| nb.parse().unwrap())
            .next_tuple()
            .unwrap();
        rules.entry(page_n1).or_default().insert(page_n2);
    }
    let sum_result = to_print_str.lines().fold(0, |acc, line| {
        let values = line.split(',');
        let mut page_list: SmallVec<[u8; 24]> = smallvec![];
        let mut page_list_hash: FxHashSet<u8> = FxHashSet::default();
        for val in values {
            let value = val.parse().unwrap();
            page_list.push(value);
            page_list_hash.insert(value);
        }
        if check_page_order(&rules, &page_list, &page_list_hash) {
            return acc + page_list[page_list.len() / 2] as u32;
        }
        acc
    });
    Some(sum_result)
}

pub fn find_page_order(
    rules: &FxHashMap<u8, FxHashSet<u8>>,
    page_list: &[u8],
    page_list_hash: &FxHashSet<u8>,
) -> SmallVec<[u8; 24]> {
    let mut order_list: SmallVec<[(u8, u8); 24]> = smallvec![];

    for &page_nb in page_list {
        let common = rules
            .get(&page_nb)
            .unwrap_or(&FxHashSet::default())
            .intersection(page_list_hash)
            .count() as u8;
        order_list.push((common, page_nb));
    }
    order_list.sort_unstable();
    order_list.reverse();
    let (_, pages_new): (SmallVec<[u8; 24]>, SmallVec<[u8; 24]>) =
        order_list.iter().copied().unzip();
    pages_new
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rules: FxHashMap<u8, FxHashSet<u8>> = FxHashMap::default();
    let (rules_str, to_print_str) = input.split("\n\n").next_tuple().unwrap();
    for line in rules_str.lines() {
        let (page_n1, page_n2) = line
            .split('|')
            .map(|nb| nb.parse().unwrap())
            .next_tuple()
            .unwrap();
        rules.entry(page_n1).or_default().insert(page_n2);
    }
    let sum_result = to_print_str.lines().fold(0, |acc, line| {
        let values = line.split(',');
        let mut page_list: SmallVec<[u8; 24]> = smallvec![];
        let mut page_list_hash: FxHashSet<u8> = FxHashSet::default();
        for val in values {
            let value = val.parse().unwrap();
            page_list.push(value);
            page_list_hash.insert(value);
        }
        let new_page_order = find_page_order(&rules, &page_list, &page_list_hash);
        if page_list != new_page_order {
            return acc + new_page_order[new_page_order.len() / 2] as u32;
        }
        acc
    });
    Some(sum_result)
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
        assert_eq!(result, Some(123));
    }
}
