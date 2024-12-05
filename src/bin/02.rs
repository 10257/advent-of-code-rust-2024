use itertools::Itertools;
use smallvec::{smallvec, SmallVec};

advent_of_code::solution!(2);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn is_safe(report: &[i32]) -> bool {
    let direction = report[0] >= report[1];
    let range = 1..=3;
    return report.iter().tuple_windows().all(|(&a, &b)| {
        // println!("{} {} {:?}", a, b, ((direction && a > b) || (!direction && a < b)) && range.contains(&a.abs_diff(b)));
        ((direction && a > b) || (!direction && a < b)) && range.contains(&a.abs_diff(b))
    });
}

pub fn part_one(input: &str) -> Option<u32> {
    // rustier but slower... is there an another way?
    let safe_report = input
        .lines()
        .filter(|&line| {
            // this is the slower:
            // let report = line
            //     .split_ascii_whitespace()
            //     .map(|nb| nb.parse::<i32>().unwrap())
            //     .collect::<SmallVec<[i32; 10]>>();
            // than this
            let values = line.split_ascii_whitespace();
            let mut report: SmallVec<[i32; 10]> = smallvec![];
            for val in values {
                report.push(val.parse::<i32>().unwrap());
            }
            // end here
            is_safe(&report)
        })
        .count() as u32;

    Some(safe_report)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_report = input
        .lines()
        .filter(|&line| {
            // this is the slower:
            let report = line
                .split_ascii_whitespace()
                .map(|nb| nb.parse::<i32>().unwrap())
                .collect::<SmallVec<[i32; 10]>>();
            // than this
            // let values = line.split_ascii_whitespace();
            // let mut report: SmallVec<[i32; 10]> = smallvec![];
            // for val in values {
            //     report.push(val.parse::<i32>().unwrap());
            // }
            // end here
            if !is_safe(&report) {
                let report_len = report.len();
                for i in 0..report_len {
                    let mut report_alt = report.clone();
                    report_alt.remove(i);
                    if is_safe(&report_alt) {
                        // println!("alt safe\t{:?}", report_alt);
                        // println!("for unsafe\t{:?}", report);
                        return true;
                    }
                }
            } else {
                return true;
                // println!("safe\t{:?}", report);
            }
            false
        })
        .count() as u32;

    Some(safe_report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
