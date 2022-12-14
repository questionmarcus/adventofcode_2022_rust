use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
enum Operation {
    Noop,
    AddX(i32),
}

const CLOCK_CYCLE_TO_MEASURE: [usize; 6] = [20, 60, 100, 140, 180, 220];

trait ProcessorObserver {
    fn update(&mut self, clock_cycle: usize, register_value: i32);
}

struct Processor<'a, T: ProcessorObserver> {
    cycle_count: usize,
    register_value: i32,
    observer: &'a mut T,
}

struct SignalStrengthStore(i32);

pub struct Screen([char; 240]);

impl ProcessorObserver for SignalStrengthStore {
    fn update(&mut self, clock_cycle: usize, register_value: i32) {
        if CLOCK_CYCLE_TO_MEASURE.contains(&clock_cycle) {
            self.0 += clock_cycle as i32 * register_value;
        }
    }
}

impl ProcessorObserver for Screen {
    fn update(&mut self, clock_cycle: usize, register_value: i32) {
        let horizontal_pixel_count: i32 = (clock_cycle % 40).try_into().unwrap();
        if (register_value - 1..=register_value + 1).contains(&(horizontal_pixel_count - 1)) {
            if let Some(pixel) = self.0.get_mut(clock_cycle - 1) {
                *pixel = '#'
            };
        }
    }
}

impl Screen {
    fn new() -> Self {
        Screen(['.'; 240])
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        writeln!(f)?;
        writeln!(f, "        {}", self.0[0..40].iter().collect::<String>())?;
        writeln!(f, "        {}", self.0[40..80].iter().collect::<String>())?;
        writeln!(f, "        {}", self.0[80..120].iter().collect::<String>())?;
        writeln!(f, "        {}", self.0[120..160].iter().collect::<String>())?;
        writeln!(f, "        {}", self.0[160..200].iter().collect::<String>())?;
        writeln!(f, "        {}", self.0[200..240].iter().collect::<String>())
    }
}

impl<'a, T: ProcessorObserver> Processor<'a, T> {
    pub fn new(observer: &'a mut T) -> Self {
        let processor = Processor {
            cycle_count: 1,
            register_value: 1,
            observer,
        };

        processor.observer.update(1, 1);

        processor
    }

    pub fn execute_operation(&mut self, op: Operation) {
        match op {
            Operation::Noop => self.increment_clock_cycle(),
            Operation::AddX(val) => {
                self.increment_clock_cycle();
                self.register_value += val;
                self.increment_clock_cycle();
            }
        }
    }

    fn increment_clock_cycle(&mut self) {
        self.cycle_count += 1;
        self.observer.update(self.cycle_count, self.register_value);
    }
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        if input == "noop" {
            return Operation::Noop;
        } else {
            let (_, add_val) = input.split_at(4);
            return Operation::AddX(add_val.trim().parse::<i32>().unwrap());
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let mut signal = SignalStrengthStore(0);
    let mut processor = Processor::new(&mut signal);

    input
        .lines()
        .map(Operation::from)
        .for_each(|op| processor.execute_operation(op));

    signal.0
}

pub fn part_two(input: &str) -> Screen {
    let mut screen = Screen::new();
    let mut processor = Processor::new(&mut screen);

    input
        .lines()
        .map(Operation::from)
        .for_each(|op| processor.execute_operation(op));

    screen
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 13140)
    }

    #[test]
    fn part_two_works() {
        assert_eq!(
            part_two(INPUT).0.iter().collect::<String>(),
            EXPECTED_SCREEN_CHARS
        );
    }

    #[test]
    fn parsing_operation() {
        assert_eq!(Operation::from("noop"), Operation::Noop);
        assert_eq!(Operation::from("addx 8"), Operation::AddX(8));
        assert_eq!(Operation::from("addx -99"), Operation::AddX(-99));
    }

    #[test]
    fn observability_with_noop_works() {
        let mut signal = SignalStrengthStore(1);
        let mut processor = Processor {
            cycle_count: 18,
            register_value: 0,
            observer: &mut signal,
        };

        processor.execute_operation(Operation::Noop);

        assert_eq!(processor.cycle_count, 19);
        assert_eq!(signal.0, 1);
    }

    #[test]
    fn observability_with_add_works() {
        let mut signal = SignalStrengthStore(0);
        let mut processor = Processor {
            cycle_count: 19,
            register_value: 1,
            observer: &mut signal,
        };

        processor.execute_operation(Operation::AddX(9));

        assert_eq!(processor.cycle_count, 21);
        assert_eq!(processor.register_value, 10);
        assert_eq!(signal.0, 20);
    }

    #[test]
    fn observability_with_screen_works() {
        let mut screen = Screen::new();
        let mut processor = Processor::new(&mut screen);

        processor.execute_operation(Operation::Noop);

        assert_eq!(screen.0[0], '#')
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const EXPECTED_SCREEN_CHARS: &str = "##..##..##..##..##..##..##..##..##..##..###...###...###...###...###...###...###.####....####....####....####....####....#####.....#####.....#####.....#####.....######......######......######......###.#######.......#######.......#######.....";
}
