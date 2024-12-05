advent_of_code::solution!(4);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|str| str.as_bytes()).collect();
    let mut nb_match: u32 = 0;
    let len = grid.len();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == b'X' {
                if j + 3 < len {
                    if grid[i][j..=j + 3] == [b'X', b'M', b'A', b'S'] {
                        // EAST =>
                        nb_match += 1;
                    }
                    if i + 3 < len // SOUTH EAST =>
                        && [
                            grid[i][j],
                            grid[i + 1][j + 1],
                            grid[i + 2][j + 2],
                            grid[i + 3][j + 3],
                        ] == [b'X', b'M', b'A', b'S']
                    {
                        nb_match += 1;
                    }
                }
                if j >= 3 {
                    if grid[i][j - 3..=j] == [b'S', b'A', b'M', b'X'] {
                        // <= WEST
                        nb_match += 1;
                    }
                    if i >= 3 // <= NORTH WEST
                        && [
                            grid[i - 3][j - 3],
                            grid[i - 2][j - 2],
                            grid[i - 1][j - 1],
                            grid[i][j],
                        ] == [b'S', b'A', b'M', b'X']
                    {
                        nb_match += 1;
                    }
                }
                if i + 3 < len {
                    if [grid[i][j], grid[i + 1][j], grid[i + 2][j], grid[i + 3][j]]
                        == [b'X', b'M', b'A', b'S']
                    {
                        // SOUTH
                        nb_match += 1;
                    }
                    if j >= 3 // SOUTH WEST
                        && [
                            grid[i + 3][j - 3],
                            grid[i + 2][j - 2],
                            grid[i + 1][j - 1],
                            grid[i][j],
                        ] == [b'S', b'A', b'M', b'X']
                    {
                        nb_match += 1;
                    }
                }
                if i >= 3 {
                    if [grid[i - 3][j], grid[i - 2][j], grid[i - 1][j], grid[i][j]]
                        == [b'S', b'A', b'M', b'X']
                    {
                        // NORTH
                        nb_match += 1;
                    }
                    if j + 3 < len // NORTH EAST
                        && [
                            grid[i][j],
                            grid[i - 1][j + 1],
                            grid[i - 2][j + 2],
                            grid[i - 3][j + 3],
                        ] == [b'X', b'M', b'A', b'S']
                    {
                        nb_match += 1;
                    }
                }
            }
        }
    }

    Some(nb_match)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|str| str.as_bytes()).collect();
    let mut nb_match: u32 = 0;
    let len = grid.len();
    for i in 0..len {
        for j in 0..len {
            if grid[i][j] == b'A' {
                if i + 1 < len
                    && i >= 1
                    && j + 1 < len
                    && j >= 1
                    && ([
                        grid[i - 1][j - 1],
                        grid[i - 1][j + 1],
                        grid[i + 1][j - 1],
                        grid[i + 1][j + 1],
                    ] == [b'S', b'S', b'M', b'M']
                        || [
                            grid[i - 1][j - 1],
                            grid[i - 1][j + 1],
                            grid[i + 1][j - 1],
                            grid[i + 1][j + 1],
                        ] == [b'M', b'S', b'M', b'S']
                        || [
                            grid[i - 1][j - 1],
                            grid[i - 1][j + 1],
                            grid[i + 1][j - 1],
                            grid[i + 1][j + 1],
                        ] == [b'S', b'M', b'S', b'M']
                        || [
                            grid[i - 1][j - 1],
                            grid[i - 1][j + 1],
                            grid[i + 1][j - 1],
                            grid[i + 1][j + 1],
                        ] == [b'M', b'M', b'S', b'S'])
                {
                    nb_match += 1;
                }
            }
        }
    }

    Some(nb_match)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
