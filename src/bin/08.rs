use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(8);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    pub fn antinodes(&self, center: &Point, width: &usize) -> Option<Point> {
        let anti = Point {
            x: center.x + (center.x - self.x),
            y: center.y + (center.y - self.y),
        };
        if anti.x < 0 || anti.y < 0 || anti.x >= (*width as i16) || anti.y >= (*width as i16) {
            return None;
        }
        Some(anti)
    }
}

pub fn make_antenna_db(input: &str) -> (FxHashMap<u8, Vec<Point>>, usize) {
    let mut antennaes: FxHashMap<u8, Vec<Point>> = FxHashMap::default();
    let mut width = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        if width == 0 {
            width = line.len();
        }
        line.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(x, &char)| match char {
                b'.' => (),
                _ => antennaes.entry(char).or_default().push(Point {
                    x: x as i16,
                    y: y as i16,
                }),
            });
    });
    (antennaes, width)
}

pub fn find_antinodes<F>(antennaes: FxHashMap<u8, Vec<Point>>, mut anti_f: F)
where
    F: FnMut(&Point, &Point),
{
    antennaes.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos| {
            positions.iter().for_each(|point| {
                anti_f(point, pos);
            });
        });
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    let (antennaes, width) = make_antenna_db(input);
    // println!("{:#?}", antennaes);
    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    find_antinodes(antennaes, |point, center| {
        if point == center {
            return;
        }
        if let Some(anti) = point.antinodes(center, &width) {
            // println!("{:?}", anti);
            antinodes.insert(anti);
        }
    });
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (antennaes, width) = make_antenna_db(input);
    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    find_antinodes(antennaes, |point, center| {
        if point == center {
            antinodes.insert(*point);
            return;
        }
        let mut center_cpy = *center;
        let mut point_cpy = *point;
        while let Some(anti) = point_cpy.antinodes(&center_cpy, &width) {
            // println!("{:?}", anti);
            antinodes.insert(anti);
            point_cpy = center_cpy;
            center_cpy = anti;
        }
    });
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
