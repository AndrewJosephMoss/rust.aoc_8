use std::collections::HashSet;

pub fn process_part_1(input: &str) -> usize {
    let mut coords = HashSet::<(usize, usize)>::new();
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| extract_row_of_heights(&line))
        .collect();
    grid.iter().enumerate().for_each(|(i, row)| {
        let visible = get_visible_tree_idxs(row);
        visible.iter().for_each(|j| {
            coords.insert((i, *j));
        })
    });
    let transposed = transpose_grid(&grid);
    transposed.iter().enumerate().for_each(|(j, row)| {
        let visible = get_visible_tree_idxs(row);
        visible.iter().for_each(|i| {
            coords.insert((*i, j));
        })
    });
    coords.len()
}

fn extract_row_of_heights(row: &str) -> Vec<u32> {
    let heights: Vec<u32> = row.chars().map(|c| c.to_digit(10).unwrap()).collect();
    heights
}

fn transpose_grid(grid: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut transposed: Vec<Vec<u32>> = vec![vec![0; grid.len()]; grid[0].len()];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            transposed[j][i] = grid[i][j];
        }
    }
    transposed
}

fn get_visible_tree_idxs(row: &Vec<u32>) -> HashSet<usize> {
    let row = row.clone().iter().map(|v| *v as i32).collect::<Vec<i32>>();
    let mut prev_highest: i32 = -1;
    let mut idxs_of_visible_trees = HashSet::<usize>::new();
    row.iter().enumerate().for_each(|(i, height)| {
        if height > &prev_highest {
            prev_highest = *height;
            idxs_of_visible_trees.insert(i);
        }
    });
    prev_highest = -1;
    row.iter().enumerate().rev().for_each(|(i, height)| {
        if height > &prev_highest {
            prev_highest = *height;
            idxs_of_visible_trees.insert(i);
        }
    });
    idxs_of_visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn text_transpose_grid() {
        let input = vec![vec![0, 0, 0], vec![1, 1, 1], vec![2, 2, 2]];
        let result = transpose_grid(&input);
        let expected = vec![vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_count_visible_trees() {
        let input: Vec<u32> = vec![3, 0, 3, 7, 3];
        let result = get_visible_tree_idxs(&input);
        let expected: HashSet<usize> = vec![0, 3, 4].into_iter().collect();
        assert_eq!(result, expected);

        let input: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
        let result = get_visible_tree_idxs(&input);
        let expected: HashSet<usize> = vec![0, 1, 2, 3, 4, 5].into_iter().collect();
        assert_eq!(result, expected);

        let input: Vec<u32> = vec![1, 4, 2, 0, 1];
        let result = get_visible_tree_idxs(&input);
        let expected: HashSet<usize> = vec![0, 1, 2, 4].into_iter().collect();
        assert_eq!(result, expected);

        let input: Vec<u32> = vec![3, 5, 3, 9, 0];
        let result = get_visible_tree_idxs(&input);
        let expected: HashSet<usize> = vec![0, 1, 3, 4].into_iter().collect();
        assert_eq!(result, expected);

        let input: Vec<u32> = vec![3, 3, 5, 4, 9];
        let result = get_visible_tree_idxs(&input);
        let expected: HashSet<usize> = vec![0, 2, 4].into_iter().collect();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_heights() {
        let input = "30373";
        let result = extract_row_of_heights(&input);
        let expected: Vec<u32> = vec![3, 0, 3, 7, 3];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_process_part_1() {
        let input = fs::read_to_string("test-input1.txt").unwrap();
        let result = process_part_1(&input);
        assert_eq!(result, 21);
    }
}
