pub fn part_one(input: &str) -> u32 {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u32 {
    let mut calories_per_elf: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.trim().parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    calories_per_elf.sort_by(|a, b| b.cmp(a));

    calories_per_elf.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 24000);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 45000);
    }
}
