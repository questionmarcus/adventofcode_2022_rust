use std::fs::read_to_string;

pub mod day01;
pub mod day02;
pub mod day03;

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
}
