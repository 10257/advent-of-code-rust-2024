use std::collections::VecDeque;

use rustc_hash::FxHashSet;

advent_of_code::solution!(10);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn add_visitable(
    map: &[Vec<u32>],
    map_size: usize,
    (x, y): (usize, usize),
    visited: &mut FxHashSet<(usize, usize)>,
    to_visit: &mut VecDeque<(usize, usize)>,
) {
    let val = map[y][x] + 1;
    
    let dirs: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    for dir in dirs {
        let (x_new, y_new) = (x as i32 + dir.0, y as i32 + dir.1);
        if x_new < 0 || x_new >= map_size as i32 || y_new < 0 || y_new >= map_size as i32 {
            continue;
        }
        if map[y_new as usize][x_new as usize] == val && !visited.contains(&(x_new as usize, y_new as usize)) {
            to_visit.push_back((x_new as usize, y_new as usize));
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
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
    let map_size = topo_map.len();
    let result = trail_head.iter().fold(0, |acc, (x, y)| {
        let mut visited: FxHashSet<(usize, usize)> = FxHashSet::default();
        let mut to_visit: VecDeque<(usize, usize)> = VecDeque::new();
        let mut nine: FxHashSet<(usize, usize)> = FxHashSet::default();
        visited.insert((*x, *y));
        add_visitable(&topo_map, map_size, (*x, *y), &mut visited, &mut to_visit);
        while !to_visit.is_empty() {
            let (a, b) = to_visit.pop_front().unwrap();
            visited.insert((a, b));
            if topo_map[b][a] == 9 {
                nine.insert((a, b));
            } else {
                add_visitable(&topo_map, map_size, (a, b), &mut visited, &mut to_visit);
            }
        }
        acc + nine.len()
    });
    Some(result as u32)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
