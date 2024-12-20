use std::collections::VecDeque;

use rustc_hash::FxHashSet;

advent_of_code::solution!(10);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn parse_topomap(input: &str) -> (Vec<Vec<u32>>, Vec<(usize, usize)>) {
    let mut trail_head: Vec<(usize, usize)> = vec![];
    let topo_map: Vec<Vec<u32>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, height_c)| {
                    let height = height_c.to_digit(10).unwrap();
                    if height == 0 {
                        trail_head.push((x, y));
                    }
                    height
                })
                .collect()
        })
        .collect();
    (topo_map, trail_head)
}

pub fn add_visitable(
    map: &[Vec<u32>],
    map_size: usize,
    (x, y): (usize, usize),
    to_visit: &mut VecDeque<(usize, usize)>,
) {
    let val = map[y][x] + 1;

    for dir in DIRS {
        if let Some((x_new, y_new)) = x.checked_add_signed(dir.0).zip(y.checked_add_signed(dir.1)) {
            if x_new >= map_size || y_new >= map_size {
                continue;
            }
            if map[y_new][x_new] == val {
                to_visit.push_back((x_new, y_new));
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (topo_map, trail_head) = parse_topomap(input);
    let map_size = topo_map.len();
    let result = trail_head
        .iter()
        .map(|(x, y)| {
            let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
            let mut nine: FxHashSet<(usize, usize)> = FxHashSet::default();
            add_visitable(&topo_map, map_size, (*x, *y), &mut to_visit);
            while !to_visit.is_empty() {
                let (a, b) = to_visit.pop_front().unwrap();
                if topo_map[b][a] == 9 {
                    nine.insert((a, b));
                } else {
                    add_visitable(&topo_map, map_size, (a, b), &mut to_visit);
                }
            }
            nine.len() as u32
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (topo_map, trail_head) = parse_topomap(input);
    let map_size = topo_map.len();
    let result = trail_head
        .iter()
        .map(|(x, y)| {
            let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
            let mut nine: u32 = 0;
            add_visitable(&topo_map, map_size, (*x, *y), &mut to_visit);
            while !to_visit.is_empty() {
                let (a, b) = to_visit.pop_front().unwrap();
                if topo_map[b][a] == 9 {
                    nine += 1;
                } else {
                    add_visitable(&topo_map, map_size, (a, b), &mut to_visit);
                }
            }
            nine
        })
        .sum();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
