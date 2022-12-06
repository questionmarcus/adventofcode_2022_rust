use std::collections::{HashSet, VecDeque};

fn get_index_after_n_unique_chars(input: &str, n_unique_chars: usize) -> usize {
    let mut last_n_chars: VecDeque<char> = VecDeque::new();

    let input_iter = input.trim().chars().enumerate();

    let mut index_after_n_unique_chars: usize = 0;

    for (signal_index, signal) in input_iter {
        if last_n_chars.len() == n_unique_chars {
            let all_unique: bool =
                HashSet::<&char>::from_iter(last_n_chars.iter()).len() == n_unique_chars;
            if all_unique {
                index_after_n_unique_chars = signal_index;
                break;
            } else {
                last_n_chars.pop_back();
            }
        }

        last_n_chars.push_front(signal)
    }

    index_after_n_unique_chars
}

pub fn part_one(input: &str) -> usize {
    get_index_after_n_unique_chars(input, 4)
}

pub fn part_two(input: &str) -> usize {
    get_index_after_n_unique_chars(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part_one("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part_one("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part_one("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part_two("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part_two("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part_two("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part_two("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
