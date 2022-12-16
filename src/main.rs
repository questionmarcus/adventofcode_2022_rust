use std::fs::read_to_string;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    for (day_index, (part_one_output, part_two_output)) in
        get_problem_output_strings().iter().enumerate()
    {
        println!("########## Day {:02} ##########", day_index + 1);
        println!("> Part 1 solution: {}", part_one_output);
        println!("> Part 2 solution: {}", part_two_output);
    }
}

fn get_problem_output_strings() -> Vec<(String, String)> {
    vec![
        (
            day01::part_one(&read_to_string("src/data/day01.txt").unwrap()).to_string(),
            day01::part_two(&read_to_string("src/data/day01.txt").unwrap()).to_string(),
        ),
        (
            day02::part_one(&read_to_string("src/data/day02.txt").unwrap()).to_string(),
            day02::part_two(&read_to_string("src/data/day02.txt").unwrap()).to_string(),
        ),
        (
            day03::part_one(&read_to_string("src/data/day03.txt").unwrap()).to_string(),
            day03::part_two(&read_to_string("src/data/day03.txt").unwrap()).to_string(),
        ),
        (
            day04::part_one(&read_to_string("src/data/day04.txt").unwrap()).to_string(),
            day04::part_two(&read_to_string("src/data/day04.txt").unwrap()).to_string(),
        ),
        (
            day05::part_one(&read_to_string("src/data/day05.txt").unwrap()),
            day05::part_two(&read_to_string("src/data/day05.txt").unwrap()),
        ),
        (
            day06::part_one(&read_to_string("src/data/day06.txt").unwrap()).to_string(),
            day06::part_two(&read_to_string("src/data/day06.txt").unwrap()).to_string(),
        ),
        (
            day07::part_one(&read_to_string("src/data/day07.txt").unwrap()).to_string(),
            day07::part_two(&read_to_string("src/data/day07.txt").unwrap()).to_string(),
        ),
        (
            day08::part_one(&read_to_string("src/data/day08.txt").unwrap()).to_string(),
            day08::part_two(&read_to_string("src/data/day08.txt").unwrap()).to_string(),
        ),
        (
            day09::part_one(&read_to_string("src/data/day09.txt").unwrap()).to_string(),
            day09::part_two(&read_to_string("src/data/day09.txt").unwrap()).to_string(),
        ),
        (
            day10::part_one(&read_to_string("src/data/day10.txt").unwrap()).to_string(),
            day10::part_two(&read_to_string("src/data/day10.txt").unwrap()).to_string(),
        ),
        (
            day11::part_one(&read_to_string("src/data/day11.txt").unwrap()).to_string(),
            day11::part_two(&read_to_string("src/data/day11.txt").unwrap()).to_string(),
        ),
    ]
}
