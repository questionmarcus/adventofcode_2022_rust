use std::collections::{HashSet, VecDeque};

type Coordinate = (i32, i32);

#[derive(PartialEq, Debug)]
enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Debug)]
struct Instruction {
    direction: MoveDirection,
    count: usize,
}

#[derive(Debug)]
struct RopePosition {
    knot_positions: VecDeque<Coordinate>,
}

impl RopePosition {
    pub fn new(rope_segments: usize) -> Self {
        let mut knot_positions: VecDeque<Coordinate> = VecDeque::new();
        for _ in 0..=rope_segments {
            knot_positions.push_front((0, 0))
        }
        RopePosition { knot_positions }
    }

    pub fn move_rope_head(&mut self, direction: &MoveDirection) {
        let mut new_head = self.knot_positions.pop_front().unwrap();
        match direction {
            MoveDirection::Down => new_head.1 -= 1,
            MoveDirection::Up => new_head.1 += 1,
            MoveDirection::Left => new_head.0 -= 1,
            MoveDirection::Right => new_head.0 += 1,
        };

        self.knot_positions.push_front(new_head);

        let mut segment_start = new_head;
        for segment_end in self.knot_positions.iter_mut().skip(1) {
            let dx = segment_start.0 - segment_end.0;
            let dy = segment_start.1 - segment_end.1;
            let segment_length: f32 = ((dx as f32).powi(2) + (dy as f32).powi(2)).sqrt();
            if segment_length >= 2.0 {
                segment_end.0 += dx.signum();
                segment_end.1 += dy.signum();
            }
            segment_start = *segment_end;
        }
    }
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let (dir, count) = line.trim().split_once(' ').unwrap();

        let parsed_count: usize = count.parse().unwrap();
        match dir {
            "U" => Instruction {
                direction: MoveDirection::Up,
                count: parsed_count,
            },
            "D" => Instruction {
                direction: MoveDirection::Down,
                count: parsed_count,
            },
            "L" => Instruction {
                direction: MoveDirection::Left,
                count: parsed_count,
            },
            "R" => Instruction {
                direction: MoveDirection::Right,
                count: parsed_count,
            },
            _ => panic!("invalid direction provided"),
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut positions = RopePosition::new(1);

    let mut tail_positions: HashSet<Coordinate> = HashSet::new();

    tail_positions.insert(*positions.knot_positions.back().unwrap());

    for line in input.lines() {
        let instruction = Instruction::from(line);
        for _ in 0..instruction.count {
            positions.move_rope_head(&instruction.direction);
            tail_positions.insert(*positions.knot_positions.back().unwrap());
        }
    }

    tail_positions.len()
}

pub fn part_two(input: &str) -> usize {
    let mut positions = RopePosition::new(9);

    let mut tail_positions: HashSet<Coordinate> = HashSet::new();

    tail_positions.insert(*positions.knot_positions.back().unwrap());

    for line in input.lines() {
        let instruction = Instruction::from(line);
        for _ in 0..instruction.count {
            positions.move_rope_head(&instruction.direction);
            tail_positions.insert(*positions.knot_positions.back().unwrap());
        }
    }

    tail_positions.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 13)
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 1)
    }

    #[test]
    fn parsing_instruction() {
        assert_eq!(
            Instruction::from("R 12"),
            Instruction {
                direction: MoveDirection::Right,
                count: 12
            }
        );
        assert_eq!(
            Instruction::from("L 4"),
            Instruction {
                direction: MoveDirection::Left,
                count: 4
            }
        );
        assert_eq!(
            Instruction::from("U 9"),
            Instruction {
                direction: MoveDirection::Up,
                count: 9
            }
        );
        assert_eq!(
            Instruction::from("D 2"),
            Instruction {
                direction: MoveDirection::Down,
                count: 2
            }
        );
    }
}
