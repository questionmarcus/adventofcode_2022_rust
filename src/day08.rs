struct Forest {
    width: u32,
    length: u32,
    rows: Vec<Vec<u8>>,
    columns: Vec<Vec<u8>>,
}

impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        let mut rows: Vec<Vec<u8>> = Vec::new();
        let mut columns: Vec<Vec<u8>> = Vec::new();

        for line in input.lines() {
            let heights: Vec<u8> = line
                .trim()
                .split("")
                .filter_map(|char| char.parse::<u8>().ok())
                .collect();

            if columns.is_empty() {
                for _ in 0..heights.len() {
                    columns.push(Vec::new())
                }
            }

            heights
                .iter()
                .enumerate()
                .for_each(|(index, height)| columns[index].push(*height));

            rows.push(heights);
        }

        Forest {
            width: columns.len() as u32,
            length: rows.len() as u32,
            rows,
            columns,
        }
    }
}

impl Forest {
    fn get_perimiter_tree_count(self) -> u32 {
        (self.length + self.width) * 2 - 4
    }

    pub fn get_visible_tree_count(self) -> u32 {
        let mut visible_tree_count: u32 = 0;

        for row_index in 1..self.width - 1 {
            for col_index in 1..self.length - 1 {
                let current_tree_height = self.rows[row_index as usize][col_index as usize];

                let current_tree_is_taller = |tree_height: &u8| tree_height < &current_tree_height;

                let visible_left = self.rows[row_index as usize]
                    .iter()
                    .take(col_index as usize)
                    .all(current_tree_is_taller);
                let visible_right = self.rows[row_index as usize]
                    .iter()
                    .skip((col_index + 1) as usize)
                    .all(current_tree_is_taller);
                let visible_up = self.columns[col_index as usize]
                    .iter()
                    .take(row_index as usize)
                    .all(current_tree_is_taller);
                let visible_down = self.columns[col_index as usize]
                    .iter()
                    .skip((row_index + 1) as usize)
                    .all(current_tree_is_taller);

                if visible_left || visible_right || visible_up || visible_down {
                    visible_tree_count += 1;
                }
            }
        }

        self.get_perimiter_tree_count() + visible_tree_count
    }

    pub fn get_max_scenic_score(self) -> u32 {
        let mut scenic_scores: Vec<u32> = Vec::new();

        for row_index in 1..self.width - 1 {
            for col_index in 1..self.length - 1 {
                let current_tree_height = self.rows[row_index as usize][col_index as usize];

                let tree_count_left = match self.rows[row_index as usize]
                    .iter()
                    .take(col_index as usize)
                    .rev()
                    .position(|height| height >= &current_tree_height)
                {
                    Some(index) => (index as u32) + 1,
                    None => col_index,
                };

                let tree_count_right = match self.rows[row_index as usize]
                    .iter()
                    .skip(col_index as usize + 1)
                    .position(|height| height >= &current_tree_height)
                {
                    Some(index) => (index as u32) + 1,
                    None => self.width - col_index - 1,
                };

                let tree_count_up = match self.columns[col_index as usize]
                    .iter()
                    .take(row_index as usize)
                    .rev()
                    .position(|height| height >= &current_tree_height)
                {
                    Some(index) => (index as u32) + 1,
                    None => row_index,
                };

                let tree_count_down = match self.columns[col_index as usize]
                    .iter()
                    .skip(row_index as usize + 1)
                    .position(|height| height >= &current_tree_height)
                {
                    Some(index) => (index as u32) + 1,
                    None => self.length - row_index - 1,
                };

                scenic_scores
                    .push(tree_count_left * tree_count_right * tree_count_down * tree_count_up)
            }
        }

        *scenic_scores.iter().max().unwrap()
    }
}

pub fn part_one(input: &str) -> u32 {
    Forest::from(input).get_visible_tree_count()
}

pub fn part_two(input: &str) -> u32 {
    Forest::from(input).get_max_scenic_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_one_works() {
        assert_eq!(part_one(INPUT), 21)
    }

    #[test]
    fn part_two_works() {
        assert_eq!(part_two(INPUT), 8)
    }

    #[test]
    fn forest_from_str() {
        let forest = Forest::from(INPUT);
        assert_eq!(forest.length, 5);
        assert_eq!(forest.width, 5);
        assert_eq!(forest.rows[0], vec![3, 0, 3, 7, 3]);
        assert_eq!(forest.columns[0], vec![3, 2, 6, 3, 3]);
    }

    #[test]
    fn perimiter_count() {
        let forest = Forest::from(INPUT);
        assert_eq!(forest.get_perimiter_tree_count(), 16);
    }
}
