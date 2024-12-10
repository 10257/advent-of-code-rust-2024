use rustc_hash::{FxHashMap, FxHashSet};

advent_of_code::solution!(8);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x:i16,
    y:i16,
}

impl Point {
    pub fn antinodes(&self, center: &Point, width: &usize) -> Option<Point> {
        let anti = Point {
            x: center.x + (center.x - self.x),
            y: center.y + (center.y - self.y)
        };
        if anti.x < 0 || anti.y < 0 || anti.x >= (*width as i16) || anti.y >= (*width as i16) {
            return None;
        }
        Some(anti)
    }
    
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut antennaes: FxHashMap<u8, Vec<Point>> = FxHashMap::default();
    let mut width = 0;
    input.lines().enumerate().for_each(|(y, line)| {
        if width == 0 {
            width = line.len();
        }
        line.as_bytes().iter().enumerate().for_each(|(x, &char)|{
            match char {
                b'.' => (),
                _ => antennaes.entry(char).or_default().push(Point{x:x as i16, y:y as i16}),
            }
        });
    });
    // println!("{:#?}", antennaes);
    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    antennaes.iter().for_each(|(_, positions)| {
        positions.iter().for_each(|pos|{
            positions.iter().filter(|&point| point != pos).for_each(|point| {
                if let Some(anti) = point.antinodes(pos, &width) {
                    // println!("{:?}", anti);
                    antinodes.insert(anti);
                }
            });
        });
    });
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
// Hello! J'ai eu ton login par Latha, qui m'a indique que vous etiez en recherche d'equipier pour tanscendance!
// Nous sommes 2 pour le moment moi (jgreau) et lcozdenm. Est-ce que vous seriez ok pour se rencontrer pour voir si ca matcherais pour travailler ensemble sur le projet?
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
        assert_eq!(result, None);
    }
}
