use std::ops::RangeInclusive;

type Ranges = (RangeInclusive<u32>, RangeInclusive<u32>);

fn get_ranges(range_pair: &str) -> Ranges {
    let mut ranges = range_pair
        .trim()
        .split(',')
        .map(|range| {
            let mut range_iter = range
                .split('-')
                .map(|val| val.parse::<u32>().unwrap())
                .take(2);
            RangeInclusive::new(range_iter.next().unwrap(), range_iter.next().unwrap())
        })
        .take(2);
    (ranges.next().unwrap(), ranges.next().unwrap())
}

fn is_fully_containing_range(input: &Ranges) -> bool {
    input.0.clone().all(|section| input.1.contains(&section))
        || input.1.clone().all(|section| input.0.contains(&section))
}

fn is_overlapping_range(input: &Ranges) -> bool {
    input.0.clone().any(|section| input.1.contains(&section))
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(get_ranges)
        .filter(is_fully_containing_range)
        .count() as u32
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(get_ranges)
        .filter(is_overlapping_range)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 2)
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 4)
    }

    #[test]
    fn get_ranges_works() {
        const TEST_VALS: [(&str, Ranges); 6] = [
            ("2-4,6-8", (2..=4, 6..=8)),
            ("2-3,4-5", (2..=3, 4..=5)),
            ("5-7,7-9", (5..=7, 7..=9)),
            ("2-8,3-7", (2..=8, 3..=7)),
            ("6-6,4-6", (6..=6, 4..=6)),
            ("2-6,4-8", (2..=6, 4..=8)),
        ];
        for (input, expected_output) in TEST_VALS {
            assert_eq!(get_ranges(input), expected_output);
        }
    }

    #[test]
    fn range_contains_other_range() {
        const ONE_CONTAINS_ALL_TWO_RANGE: Ranges = (1..=9, 4..=8);
        const TWO_CONTAINS_ALL_ONE_RANGE: Ranges = (4..=8, 1..=9);
        const NON_OVERLAPPING_RANGE: Ranges = (2..=3, 4..=5);

        assert!(is_fully_containing_range(&ONE_CONTAINS_ALL_TWO_RANGE));
        assert!(is_fully_containing_range(&TWO_CONTAINS_ALL_ONE_RANGE));
        assert!(!is_fully_containing_range(&NON_OVERLAPPING_RANGE))
    }

    #[test]
    fn range_overlaps() {
        const OVERLAPPING_RANGE: Ranges = (1..=5, 5..=9);
        const NON_OVERLAPPING_RANGE: Ranges = (2..=3, 4..=5);

        assert!(is_overlapping_range(&OVERLAPPING_RANGE));
        assert!(!is_overlapping_range(&NON_OVERLAPPING_RANGE));
    }
}
