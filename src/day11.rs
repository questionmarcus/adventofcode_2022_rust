use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum Operand {
    Plus(u64),
    Multiply(u64),
    Pow(u32),
}

#[derive(Debug, PartialEq, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operand,
    divisible_by_check: u64,
    connected_monkey_indexes: (usize, usize),
    inspection_count: u64,
}

struct PassToMonkeyInstruction(u64, usize);

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut iter = value.lines().skip(1);
        let starting_items: VecDeque<u64> = iter
            .next()
            .unwrap()
            .trim()
            .split_at(16)
            .1
            .split(", ")
            .map(|val| val.parse::<u64>().unwrap())
            .collect();

        let (op_str, val_str) = iter.next().unwrap().trim().split_at(21).1.split_at(1);

        let op_value = match val_str.trim() {
            "old" => u64::MAX,
            val => val.parse::<u64>().unwrap(),
        };

        let operation = match op_str {
            "+" => {
                if op_value == u64::MAX {
                    Operand::Multiply(2)
                } else {
                    Operand::Plus(op_value)
                }
            }
            "*" => {
                if op_value == u64::MAX {
                    Operand::Pow(2)
                } else {
                    Operand::Multiply(op_value)
                }
            }
            _ => panic!("Cannot parse operand"),
        };

        let divisible_by_check = iter
            .next()
            .unwrap()
            .trim()
            .split_at(19)
            .1
            .parse::<u64>()
            .unwrap();

        let connected_monkeys: Vec<usize> = iter
            .map(|line| match line.split_once("throw to monkey") {
                Some((_, index)) => index.trim().parse::<usize>().unwrap(),
                None => panic!(),
            })
            .collect();

        Monkey {
            items: starting_items,
            operation,
            divisible_by_check,
            connected_monkey_indexes: (connected_monkeys[0], connected_monkeys[1]),
            inspection_count: 0,
        }
    }
}

impl Monkey {
    pub fn inspect_item_part_one(&mut self) -> Option<PassToMonkeyInstruction> {
        if let Some(current_item) = self.items.pop_front() {
            let new_item_value = self.get_bordom_value(current_item, 3);
            self.inspection_count += 1;

            if new_item_value % self.divisible_by_check == 0 {
                Some(PassToMonkeyInstruction(
                    new_item_value,
                    self.connected_monkey_indexes.0,
                ))
            } else {
                Some(PassToMonkeyInstruction(
                    new_item_value,
                    self.connected_monkey_indexes.1,
                ))
            }
        } else {
            None
        }
    }

    pub fn inspect_item_part_two(&mut self) -> Option<PassToMonkeyInstruction> {
        if let Some(current_item) = self.items.pop_front() {
            let new_item_value = self.get_bordom_value(current_item, 1);
            self.inspection_count += 1;

            if new_item_value % self.divisible_by_check == 0 {
                Some(PassToMonkeyInstruction(
                    new_item_value,
                    self.connected_monkey_indexes.0,
                ))
            } else {
                Some(PassToMonkeyInstruction(
                    new_item_value,
                    self.connected_monkey_indexes.1,
                ))
            }
        } else {
            None
        }
    }

    // Returns the value trimmed to 1000. e.g. 12837647 -> 647, as
    fn get_bordom_value(&self, item_value: u64, worry_denominator: u64) -> u64 {
        match self.operation {
            Operand::Plus(amount) => (item_value + amount) / worry_denominator,
            Operand::Multiply(amount) => {
                println!("{} x {}", item_value, amount);
                (item_value * amount) / worry_denominator
            }
            Operand::Pow(amount) => (item_value.pow(amount)) / worry_denominator,
        }
    }
}

pub fn part_one(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let current_monkey = monkeys.get_mut(monkey_index).unwrap();
                let PassToMonkeyInstruction(item_value, monkey_index) =
                    current_monkey.inspect_item_part_one().unwrap();

                monkeys
                    .get_mut(monkey_index)
                    .unwrap()
                    .items
                    .push_back(item_value);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product()
}

pub fn part_two(input: &str) -> u64 {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::from).collect();

    for _round in 0..20 {
        println!("round {}", _round + 1);
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let current_monkey = monkeys.get_mut(monkey_index).unwrap();
                let PassToMonkeyInstruction(item_value, monkey_index) =
                    current_monkey.inspect_item_part_two().unwrap();

                monkeys
                    .get_mut(monkey_index)
                    .unwrap()
                    .items
                    .push_back(item_value);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspection_count.partial_cmp(&a.inspection_count).unwrap());

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspection_count)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 10605)
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 2713310158)
    }

    #[test]
    fn monkey_parsing() {
        const INPUT_1: &str = "Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0";

        const INPUT_2: &str = "Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3";

        let expected_monkey_1: Monkey = Monkey {
            items: VecDeque::from([54, 65, 75, 74]),
            operation: Operand::Plus(6),
            divisible_by_check: 19,
            connected_monkey_indexes: (2, 0),
            inspection_count: 0,
        };

        let expected_monkey_2: Monkey = Monkey {
            items: VecDeque::from([79, 60, 97]),
            operation: Operand::Pow(2),
            divisible_by_check: 13,
            connected_monkey_indexes: (1, 3),
            inspection_count: 0,
        };

        assert_eq!(Monkey::from(INPUT_1), expected_monkey_1);
        assert_eq!(Monkey::from(INPUT_2), expected_monkey_2);
    }

    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
