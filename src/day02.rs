#[derive(PartialEq, Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum GameState {
    Win,
    Draw,
    Loss,
}

struct Game {
    player_choice: Choice,
    outcome: GameState,
}

impl Game {
    pub fn from_part_one(game_arr: [&str; 2]) -> Game {
        let opponent_choice: Choice = match game_arr[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Invalid game state"),
        };

        let player_choice: Choice = match game_arr[1] {
            "X" => Choice::Rock,
            "Y" => Choice::Paper,
            "Z" => Choice::Scissors,
            _ => panic!("Invalid game state"),
        };

        let outcome = get_game_state(&opponent_choice, &player_choice);

        Game {
            player_choice,
            outcome,
        }
    }

    pub fn from_part_two(game_arr: [&str; 2]) -> Game {
        let opponent_choice: Choice = match game_arr[0] {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => panic!("Invalid game state"),
        };

        let outcome: GameState = match game_arr[1] {
            "X" => GameState::Loss,
            "Y" => GameState::Draw,
            "Z" => GameState::Win,
            _ => panic!("Invalid game state"),
        };

        let player_choice = get_player_choice(opponent_choice, &outcome);

        Game {
            player_choice,
            outcome,
        }
    }
}

fn get_player_choice(opponent_choice: Choice, outcome: &GameState) -> Choice {
    match outcome {
        GameState::Draw => opponent_choice,
        GameState::Win => match opponent_choice {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        },
        GameState::Loss => match opponent_choice {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        },
    }
}

fn get_game_state(opponent_choice: &Choice, player_choice: &Choice) -> GameState {
    match (opponent_choice, player_choice) {
        (Choice::Rock, Choice::Paper)
        | (Choice::Paper, Choice::Scissors)
        | (Choice::Scissors, Choice::Rock) => GameState::Win,
        (Choice::Rock, Choice::Scissors)
        | (Choice::Paper, Choice::Rock)
        | (Choice::Scissors, Choice::Paper) => GameState::Loss,
        _ => GameState::Draw,
    }
}

fn get_game_score(game: Game) -> u32 {
    let base_score = match game.outcome {
        GameState::Win => 6,
        GameState::Draw => 3,
        GameState::Loss => 0,
    };

    let choice_score = match game.player_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    base_score + choice_score
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let game_arr: [&str; 2] = game
                .trim()
                .split(' ')
                .take(2)
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            Game::from_part_one(game_arr)
        })
        .map(get_game_score)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|game| {
            let game_arr: [&str; 2] = game
                .trim()
                .split(' ')
                .take(2)
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            Game::from_part_two(game_arr)
        })
        .map(get_game_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 15);
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 12);
    }

    #[test]
    fn player_choice_when_draw() {
        assert_eq!(
            get_player_choice(Choice::Rock, &GameState::Draw),
            Choice::Rock
        );
        assert_eq!(
            get_player_choice(Choice::Paper, &GameState::Draw),
            Choice::Paper
        );
        assert_eq!(
            get_player_choice(Choice::Scissors, &GameState::Draw),
            Choice::Scissors
        );
    }

    #[test]
    fn player_choice_when_win() {
        assert_eq!(
            get_player_choice(Choice::Rock, &GameState::Win),
            Choice::Paper
        );
        assert_eq!(
            get_player_choice(Choice::Paper, &GameState::Win),
            Choice::Scissors
        );
        assert_eq!(
            get_player_choice(Choice::Scissors, &GameState::Win),
            Choice::Rock
        );
    }

    #[test]
    fn player_choice_when_loss() {
        assert_eq!(
            get_player_choice(Choice::Rock, &GameState::Loss),
            Choice::Scissors
        );
        assert_eq!(
            get_player_choice(Choice::Paper, &GameState::Loss),
            Choice::Rock
        );
        assert_eq!(
            get_player_choice(Choice::Scissors, &GameState::Loss),
            Choice::Paper
        );
    }
}
