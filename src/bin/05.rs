use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::{smallvec, SmallVec};

advent_of_code::solution!(5);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn check_slice(rule_set: Option<&FxHashSet<u8>>, page_list: &[u8], index: &usize) -> bool {
    let page_remain = &page_list[*index + 1..];
    if rule_set.is_none() && page_remain.len() > 1 {
        return false;
    }

    let page_remain_hash = FxHashSet::from_iter(page_remain.iter().copied());
    // println!("rule{:?} - remain{:?}", rule_set, page_remain_hash);
    if !rule_set
        .unwrap_or(&FxHashSet::default())
        .is_superset(&page_remain_hash)
    {
        return false;
    }
    let page_done_hash = FxHashSet::from_iter(page_list[..*index].iter().copied());
    // println!("rule{:?} - done{:?}", rule_set, page_done_hash);
    if !page_done_hash.is_disjoint(&rule_set.unwrap_or(&FxHashSet::default())) {
        return false;
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
        for val in values {
            page_list.push(val.parse().unwrap());
        }
        if page_list.iter().enumerate().all(|(nb, page_num)| {
            // eprintln!("len {:?} - i {:?}", page_list.len(), nb);
            check_slice(rules.get(page_num), &page_list, &nb)
        }) {
            // eprintln!("{} {}", acc, page_list[page_list.len() / 2]);
            return acc + page_list[page_list.len() / 2];
        }
        acc
    });
    Some(sum_result as u32)
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
