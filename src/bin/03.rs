use regex::Regex;

advent_of_code::solution!(3);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn part_one(input: &str) -> Option<u32> {
    let mut multiplication: u32 = 0;
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    for line in input.lines() {
        re.captures_iter(line).for_each(|c| {
            let (_, [a, b]) = c.extract();
            multiplication += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
            // println!("{} {}", a, b);
        });
    }
    Some(multiplication)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut multiplication: u32 = 0;
    let mut do_mul = true;
    let re = Regex::new(r"(do(?:n't)?\(\))?(?:.*?)?mul\(([0-9]+),([0-9]+)\)").unwrap();
    // let re = Regex::new(r"(do(?:n't)?\(\))").unwrap();
    for line in input.lines() {
        re.captures_iter(line).for_each(|c| {
            let mul = c.get(1);
            println!("{mul:?}");
            if mul.is_some() {
                let switch = mul.unwrap().as_str();
                println!("{switch:?}");
                if switch == "do()" && !do_mul {
                    do_mul = true;
                }
                if switch == "don't()" && do_mul {
                    do_mul = false;
                }
            }
            // println!("{do_mul:?}");
            // if do_mul {
            //     let (a, b) = (c.get(2).unwrap().as_str(), c.get(3).unwrap().as_str());
            //     multiplication += a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap();
            //     println!(" {} {}", a, b);
            // }
        });
    }
    println!("{multiplication:?}");
    Some(multiplication)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(48));
    }
}
