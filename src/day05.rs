struct Stacks {
    stacks: Vec<Vec<char>>,
}

struct Instruction {
    from: usize,
    to: usize,
    repeat: usize,
}

impl From<&str> for Stacks {
    fn from(input: &str) -> Self {
        let mut input_iter = input.lines().rev();
        let n_stacks: usize = input_iter.next().unwrap().trim().split("   ").count();

        let mut stacks: Vec<Vec<char>> = Vec::new();

        for _ in 0..n_stacks {
            let empty_stack: Vec<char> = Vec::new();
            stacks.push(empty_stack);
        }

        for input_row in input_iter {
            for (stack_number, item) in input_row.chars().skip(1).step_by(4).enumerate() {
                if item != ' ' {
                    stacks[stack_number].push(item)
                }
            }
        }

        Stacks { stacks }
    }
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        // Assume values always come in format: "move A from B to C"
        let mut input_values = input.split_whitespace().skip(1).step_by(2);

        Instruction {
            repeat: input_values.next().unwrap().parse().unwrap(),
            from: input_values.next().unwrap().parse::<usize>().unwrap() - 1,
            to: input_values.next().unwrap().parse::<usize>().unwrap() - 1,
        }
    }
}

impl Stacks {
    pub fn apply_instruction_part_one(&mut self, instruction: Instruction) {
        for _ in 0..instruction.repeat {
            let moved_value = self.stacks[instruction.from].pop().unwrap();
            self.stacks[instruction.to].push(moved_value);
        }
    }

    pub fn apply_instruction_part_two(&mut self, instruction: Instruction) {
        let old_stack_size = self.stacks[instruction.from].len();
        let mut moved_value =
            self.stacks[instruction.from].split_off(old_stack_size - instruction.repeat);
        self.stacks[instruction.to].append(&mut moved_value);
    }

    pub fn get_top_of_stacks(&self) -> String {
        let mut output: String = String::new();

        for stack in &self.stacks {
            match stack.len() {
                0 => (),
                n => output.push(stack[n - 1]),
            }
        }

        output
    }
}

pub fn part_one(input: &str) -> String {
    let parsed_input = input.split("\n\n").collect::<Vec<&str>>();

    let mut stacks = Stacks::from(parsed_input[0]);

    for instruction_line in parsed_input[1].lines() {
        let instruction = Instruction::from(instruction_line);
        stacks.apply_instruction_part_one(instruction);
    }

    stacks.get_top_of_stacks()
}

pub fn part_two(input: &str) -> String {
    let parsed_input = input.split("\n\n").collect::<Vec<&str>>();

    let mut stacks = Stacks::from(parsed_input[0]);

    for instruction_line in parsed_input[1].lines() {
        let instruction = Instruction::from(instruction_line);
        stacks.apply_instruction_part_two(instruction);
    }

    stacks.get_top_of_stacks()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), "CMZ")
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), "MCD")
    }

    #[test]
    fn create_stacks() {
        const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";
        let stacks = Stacks::from(INPUT);
        assert_eq!(stacks.stacks.len(), 3);
        assert_eq!(stacks.stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks.stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks.stacks[2], vec!['P'])
    }

    #[test]
    fn create_instruction() {
        const EXPECTED: Instruction = Instruction {
            from: 0,
            to: 1,
            repeat: 3,
        };
        const INPUT: &str = "move 3 from 1 to 2";
        assert_eq!(Instruction::from(INPUT).from, EXPECTED.from);
        assert_eq!(Instruction::from(INPUT).to, EXPECTED.to);
        assert_eq!(Instruction::from(INPUT).repeat, EXPECTED.repeat);
    }

    #[test]
    fn apply_instruction_part_one_to_stacks() {
        let mut test_stack: Stacks = Stacks {
            stacks: vec![vec!['A'], vec!['B']],
        };
        let instruction: Instruction = Instruction::from("move 1 from 1 to 2");

        test_stack.apply_instruction_part_one(instruction);

        assert_eq!(test_stack.stacks[0].len(), 0);
        assert_eq!(test_stack.stacks[1], vec!['B', 'A']);
    }

    #[test]
    fn apply_instruction_part_two_to_stacks() {
        let mut test_stack: Stacks = Stacks {
            stacks: vec![vec!['A', 'B'], vec!['C']],
        };
        let instruction: Instruction = Instruction::from("move 2 from 1 to 2");

        test_stack.apply_instruction_part_two(instruction);

        assert_eq!(test_stack.stacks[0].len(), 0);
        assert_eq!(test_stack.stacks[1], vec!['C', 'A', 'B']);
    }

    #[test]
    fn top_of_stacks_string() {
        let test_stack: Stacks = Stacks {
            stacks: vec![vec!['A'], vec!['B'], vec!['C', 'D']],
        };

        assert_eq!(test_stack.get_top_of_stacks(), String::from("ABD"))
    }
}
