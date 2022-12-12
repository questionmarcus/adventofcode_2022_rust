use std::collections::HashSet;

fn char_to_priority(c: char) -> u32 {
    const RADIX: u32 = 36;
    match c {
        x if x.is_ascii_uppercase() => x.to_digit(RADIX).unwrap() - 9 + 26,
        x if x.is_ascii_lowercase() => x.to_digit(RADIX).unwrap() - 9,
        _ => panic!("Invalid character supplied"),
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.trim())
        .map(|backpack| {
            let n_items_in_backpack = backpack.len();
            let (compart_1, compart_2) = backpack.split_at(n_items_in_backpack / 2);
            (
                compart_1.chars().collect::<HashSet<char>>(),
                compart_2.chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(compart_1, compart_2)| {
            char_to_priority(*compart_1.intersection(&compart_2).collect::<Vec<&char>>()[0])
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().map(|line| line.trim()).collect();
    lines
        .chunks(3)
        .map(|elf_groups| {
            let badge = elf_groups[0]
                .chars()
                .find(|item| elf_groups[1].contains(*item) && elf_groups[2].contains(*item))
                .unwrap();
            char_to_priority(badge)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  PmmdzqPrVvPwwTWBwg
  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
  ttgJtRGJQctTZtZT
  CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn char_to_points() {
        assert_eq!(char_to_priority('p'), 16);
        assert_eq!(char_to_priority('L'), 38);
        assert_eq!(char_to_priority('P'), 42);
        assert_eq!(char_to_priority('v'), 22);
        assert_eq!(char_to_priority('t'), 20);
        assert_eq!(char_to_priority('s'), 19);
    }

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 157);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 70);
    }
}
