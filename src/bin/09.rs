use itertools::{repeat_n, Itertools};

advent_of_code::solution!(9);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn part_one(input: &str) -> Option<u64> {
    let disk: Vec<u64> = input
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u64)
        .collect();
    let number_free_bloc: u64 = disk.len() as u64 / 2;
    // let total_free_spot: u64 = disk.iter().skip(1).step_by(2).sum();
    let total_file_spot: u64 = disk.iter().step_by(2).sum();

    let mut max_file_id: u64 = disk.len() as u64 - number_free_bloc - 1;
    let mut min_file_id: u64 = 0;

    let mut disk_map = disk.iter();
    let mut disk_map_rev = disk.iter().rev().step_by(2);

    let mut bloc = *disk_map.next().unwrap() as i64;
    let mut rev_file = *disk_map_rev.next().unwrap() as i64;

    let mut empty = false;
    let mut checksum = 0;
    let mut bloc_id = 0;
    loop {
        if bloc <= 0 {
            bloc = *disk_map.next().unwrap() as i64;
        }
        bloc -= 1;
        match empty {
            false => {
                if bloc >= 0 {
                    checksum += bloc_id * min_file_id;
                }
                if bloc <= 0 {
                    empty = true;
                    min_file_id += 1
                }
            }
            true => {
                if bloc >= 0 {
                    checksum += bloc_id * max_file_id;
                    rev_file -= 1;
                    if rev_file <= 0 {
                        rev_file = *disk_map_rev.next().unwrap() as i64;
                        max_file_id -= 1;
                    }
                }
                if bloc <= 0 {
                    empty = false;
                }
            }
        }
        if bloc >= 0 {
            bloc_id += 1;
        }
        if bloc_id == total_file_spot {
            break;
        }
    }
    Some(checksum)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    File,
    Free,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct File {
    type_: Type,
    size: u64,
    id: Option<u64>,
}

pub fn find_free_space(disk: &mut [File], size: u64) -> Option<usize> {
    if let Some((pos, _)) = disk
        .iter()
        .find_position(|x| x.type_ == Type::Free && x.size >= size)
    {
        Some(pos)
    } else {
        None
    }
}

pub fn find_update_pos(disk: &mut [File], id: &Option<u64>) -> Option<usize> {
    if let Some((pos, _)) = disk
        .iter()
        .find_position(|x| x.type_ == Type::File && x.id == *id)
    {
        Some(pos)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk: Vec<File> = input
        .trim_end()
        .chars()
        .enumerate()
        .map(|(id, x)| {
            let size = x.to_digit(10).unwrap() as u64;
            match id % 2 {
                0 => File {
                    type_: Type::File,
                    size,
                    id: Some(id as u64 / 2),
                },
                1 => File {
                    type_: Type::Free,
                    size,
                    id: None,
                },
                _ => File {
                    type_: Type::None,
                    size: 0,
                    id: None,
                },
            }
        })
        .collect();

    let disk_clone = disk.clone();

    disk_clone
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, f)| f.type_ == Type::File)
        .for_each(|(pos, f)| {
            let real_pos = pos + find_update_pos(&mut disk[pos..], &f.id).unwrap();
            if let Some(free_pos) = find_free_space(&mut disk[0..real_pos], f.size) {
                let free_size = disk[free_pos].size;
                disk.swap(real_pos, free_pos);
                if free_size != f.size {
                    disk.insert(
                        free_pos + 1,
                        File {
                            type_: Type::Free,
                            size: free_size - f.size,
                            id: None,
                        },
                    );
                    disk[real_pos + 1].size = f.size;
                }
            }
        });
    Some(
        disk.iter()
            .flat_map(|b| repeat_n(b.id.unwrap_or_default(), b.size as usize))
            .enumerate()
            .fold(0, |acc, (bloc_id, file_id)| acc + bloc_id as u64 * file_id),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
