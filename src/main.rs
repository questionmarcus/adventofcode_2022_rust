use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    println!("### Day 01 ###");
    println!(
        "> Part 1 solution: {}",
        day01::part_one(&read_to_string("src/data/day01.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day01::part_two(&read_to_string("src/data/day01.txt").unwrap())
    );
    println!("### Day 02 ###");
    println!(
        "> Part 1 solution: {}",
        day02::part_one(&read_to_string("src/data/day02.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day02::part_two(&read_to_string("src/data/day02.txt").unwrap())
    );
    println!("### Day 03 ###");
    println!(
        "> Part 1 solution: {}",
        day03::part_one(&read_to_string("src/data/day03.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day03::part_two(&read_to_string("src/data/day03.txt").unwrap())
    );
    println!("### Day 04 ###");
    println!(
        "> Part 1 solution: {}",
        day04::part_one(&read_to_string("src/data/day04.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day04::part_two(&read_to_string("src/data/day04.txt").unwrap())
    );
    println!("### Day 05 ###");
    println!(
        "> Part 1 solution: {}",
        day05::part_one(&read_to_string("src/data/day05.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day05::part_two(&read_to_string("src/data/day05.txt").unwrap())
    );
    println!("### Day 06 ###");
    println!(
        "> Part 1 solution: {}",
        day06::part_one(&read_to_string("src/data/day06.txt").unwrap())
    );
    println!(
        "> Part 2 solution: {}",
        day06::part_two(&read_to_string("src/data/day06.txt").unwrap())
    );
}
