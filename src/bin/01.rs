advent_of_code::solution!(1);

#[cfg(not(feature = "dhat"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut dist = line.split_ascii_whitespace();
        list1.push(dist.next().unwrap().parse::<i32>().unwrap());
        list2.push(dist.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    // let mut dist: i32 = 0;
    // // is it possible to sum the zipped iterator
    // for dist_it in list1.iter_mut().zip(list2.iter()) {
    //     let (dist1, dist2) = dist_it;
    //     dist += (*dist1 as i32 - *dist2 as i32).abs();
    // }
    // Some(dist as u32)

    // let mut dist: Vec<i32> = vec![];
    // // is it possible to sum the zipped iterator
    // for (dist1, dist2) in list1.iter_mut().zip(list2.iter()) {
    //     dist.push((*dist1 - *dist2).abs());
    // }
    // Some(dist.iter().sum::<i32>() as u32)

    // is it possible to sum the zipped iterator
    // for (dist1, dist2) in list1.iter_mut().zip(list2.iter()) {
    //     *dist1 = (*dist1 - *dist2).abs();
    // }
    // Some(list1.iter().sum::<i32>() as u32)

    let iter = list1
        .iter_mut()
        .zip(list2.iter())
        .map(|(a, b)| (*a - *b).abs());
    Some(iter.sum::<i32>() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in input.lines() {
        let mut dist = line.split_ascii_whitespace();
        list1.push(dist.next().unwrap().parse::<i32>().unwrap());
        list2.push(dist.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();
    let mut sim_score = 0;
    for value in list1 {
        sim_score += value * list2.iter().filter(|&n| *n == value).count() as i32;
    }
    Some(sim_score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}