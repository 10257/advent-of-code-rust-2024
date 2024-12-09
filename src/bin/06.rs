use std::ops::Add;

use rustc_hash::FxHashSet;

advent_of_code::solution!(6);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i16,
    y: i16,
}

impl Add<Increment> for Pos {
    type Output = Pos;

    fn add(self, rhs: Increment) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Increment {
    dir: Direction,
    x: i16,
    y: i16,
}

impl Increment {
    pub fn new(dir: Direction) -> Self {
        let (x, y) = Increment::get_incr_value(dir);
        Self { dir, x, y }
    }

    fn get_incr_value(dir: Direction) -> (i16, i16) {
        let (x, y) = match dir {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            _ => (0, 0),
        };
        (x, y)
    }

    pub fn set_dir(&mut self, dir: Direction) -> &mut Self {
        self.dir = dir;
        (self.x, self.y) = Increment::get_incr_value(dir);
        self
    }

    pub fn turn(&mut self) -> &mut Self {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => Direction::None,
        };
        (self.x, self.y) = Increment::get_incr_value(self.dir);
        self
    }
}

impl Default for Increment {
    fn default() -> Self {
        Self {
            dir: Direction::None,
            x: 0,
            y: 0,
        }
    }
}

// to remember the pain I had to find how to write into my grid...
// pub fn writeto_slice(c: &mut [Vec<u8>], guard: &Pos, s: &u8) {
//     c[guard.y as usize][guard.x as usize] = *s;
// }

pub fn print_grid(grid: &[Vec<u8>]) {
    for row in grid {
        println!("{}", String::from_utf8(row.to_vec()).unwrap());
    }
    println!("================");
}

pub fn find_guard(grid: &[Vec<u8>]) -> (Pos, Increment) {
    let mut guard_dir = Increment::default();
    let mut guard_pos = Pos { x: 0, y: 0 };
    for (r, row_b) in grid.iter().enumerate() {
        let is_guard = row_b.iter().position(|&char| char != b'.' && char != b'#');
        if is_guard.is_none() {
            continue;
        }
        guard_dir.set_dir(match row_b[is_guard.unwrap()] {
            b'^' => Direction::Up,
            b'>' => Direction::Right,
            b'v' => Direction::Down,
            b'<' => Direction::Left,
            _ => Direction::None,
        });
        guard_pos = Pos {
            x: is_guard.unwrap() as i16,
            y: r as i16,
        };
    }
    (guard_pos, guard_dir)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|str| str.as_bytes().to_vec()).collect();
    let len = grid.len() as i16;
    let (mut guard_pos, mut guard_dir) = find_guard(&grid);
    let mut counter = 1;
    grid[guard_pos.y as usize][guard_pos.x as usize] = b'X';
    loop {
        // println!("{:?} -- step: {} -- guard:{:?}", guard_dir, counter, guard_pos);
        // print_grid(&grid);
        let new_pos = guard_pos + guard_dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= len || new_pos.y >= len {
            break;
        }
        if grid[new_pos.y as usize][new_pos.x as usize] == b'#' {
            guard_dir.turn();
        } else {
            if grid[new_pos.y as usize][new_pos.x as usize] != b'X' {
                grid[new_pos.y as usize][new_pos.x as usize] = b'X';
                counter += 1;
            }
            guard_pos = new_pos;
        }
    }
    // print_grid(&grid);
    Some(counter)
}

pub fn check_alt_path(
    grid: &[Vec<u8>],
    mut guard_pos: Pos,
    mut guard_dir: Increment,
    path: &FxHashSet<(Pos, Direction)>,
) -> bool {
    let mut path_test: FxHashSet<(Pos, Direction)> = FxHashSet::default();
    path_test.insert((guard_pos, guard_dir.dir));
    let len = grid.len() as i16;
    loop {
        let new_pos = guard_pos + guard_dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= len || new_pos.y >= len {
            return false;
        }
        if grid[new_pos.y as usize][new_pos.x as usize] == b'#' {
            guard_dir.turn();
        } else {
            guard_pos = new_pos;
        }
        if path.contains(&(guard_pos, guard_dir.dir))
            || !path_test.insert((guard_pos, guard_dir.dir))
        {
            return true;
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|str| str.as_bytes().to_vec()).collect();
    let len = grid.len() as i16;
    let (mut guard_pos, mut guard_dir) = find_guard(&grid);
    // start pos
    grid[guard_pos.y as usize][guard_pos.x as usize] = b'X';
    let mut path: FxHashSet<(Pos, Direction)> = FxHashSet::default();
    let mut possible_blocs: FxHashSet<Pos> = FxHashSet::default();
    path.insert((guard_pos, guard_dir.dir));
    loop {
        // println!("{:?} -- step: {} -- guard:{:?}", guard_dir, counter, guard_pos);
        // print_grid(&grid);
        let new_pos = guard_pos + guard_dir;
        if new_pos.x < 0 || new_pos.y < 0 || new_pos.x >= len || new_pos.y >= len {
            break;
        }
        if grid[new_pos.y as usize][new_pos.x as usize] == b'#' {
            guard_dir.turn();
        } else {
            if grid[new_pos.y as usize][new_pos.x as usize] != b'X' {
                grid[new_pos.y as usize][new_pos.x as usize] = b'#';
                if check_alt_path(&grid, guard_pos, guard_dir, &path) {
                    possible_blocs.insert(new_pos);
                }
                grid[new_pos.y as usize][new_pos.x as usize] = b'X';
            }
            guard_pos = new_pos;
        }
        path.insert((guard_pos, guard_dir.dir));
    }
    Some(possible_blocs.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
