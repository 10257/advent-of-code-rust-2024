advent_of_code::solution!(2);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn is_safe(report: &Vec<i32>) -> bool
{
    let mut direction = true;
    if report[0] < report[1] {
        direction = false;
    }
    let mut safe = true;
    for values in report.windows(2) {
        let diff = values[0] - values[1];
        if (direction && diff < 0)
            || (!direction && diff > 0)
            || diff.abs() < 1
            || diff.abs() > 3
        {
            safe = false;
            break;
        }
    }
    safe
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_report = 0;
    for line in input.lines() {
        let values = line.split_ascii_whitespace();
        let mut report: Vec<i32> = vec![];
        for val in values {
            report.push(val.parse::<i32>().unwrap());
        }

        if is_safe(&report) {
            safe_report += 1;
            // println!("safe\t{:?}", report);
        }
    }
    Some(safe_report)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_report = 0;
    for line in input.lines() {
        let values = line.split_ascii_whitespace();
        let mut report: Vec<i32> = vec![];
        for val in values {
            report.push(val.parse::<i32>().unwrap());
        }

        let mut safe = is_safe(&report);

        if safe {
            safe_report += 1;
            // println!("safe\t{:?}", report);
        } else {
            let report_len = report.len();
            for i in 0..report_len {
                let mut report_alt = report.clone();
                report_alt.remove(i);
                if is_safe(&report_alt)
                {
                    // println!("alt safe\t{:?}", report_alt);
                    safe = true;
                    break;
                }
            }
            if safe {
                // println!("for unsafe\t{:?}", report);
                safe_report += 1;
            }
        }
    }
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
