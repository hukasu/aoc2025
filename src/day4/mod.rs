const EMPTY_TILE: u8 = b'.';
const PAPER_ROLL: u8 = b'@';

pub struct FloorPlan;

impl FloorPlan {
    pub fn accessible_paper_rolls(floor_plan: &[u8]) -> u32 {
        let grid = Self::gridfy_floor_plan(floor_plan);

        let adjacency_matrix = Self::build_adjacency_matrix(&grid);

        adjacency_matrix
            .iter()
            .flatten()
            .zip(grid.iter().flat_map(|row| row.iter()))
            .filter_map(|(adjacent, tile)| {
                if *tile == PAPER_ROLL && *adjacent < 4 {
                    Some(1)
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn removable_paper_rolls(floor_plan: &[u8]) -> u32 {
        let mut grid = Self::gridfy_floor_plan(floor_plan);

        let rows = grid.len();
        let columns = grid[0].len();

        let mut adjacency_matrix = Self::build_adjacency_matrix(&grid);
        let mut adjacency_matrix_b = adjacency_matrix.clone();

        let mut removed = 0;
        let mut updated = true;

        while updated {
            updated = false;
            for (row, row_b) in adjacency_matrix.iter_mut().zip(adjacency_matrix_b.iter()) {
                row.copy_from_slice(row_b);
            }

            for (row, column, tile, adjacency) in
                grid.iter_mut()
                    .zip(adjacency_matrix.iter())
                    .enumerate()
                    .flat_map(|(row_num, (row, adjacency_row))| {
                        row.iter_mut().zip(adjacency_row).enumerate().map(
                            move |(column, (tile, adjacency))| (row_num, column, tile, *adjacency),
                        )
                    })
            {
                if *tile == PAPER_ROLL && adjacency < 4 {
                    updated = true;
                    removed += 1;
                    *tile = EMPTY_TILE;

                    let left_column = column.checked_sub(1);
                    let right_column = (column + 1 < columns).then_some(column + 1);
                    let top_row = row.checked_sub(1);
                    let bottom_row = (row + 1 < rows).then_some(row + 1);

                    for (row, column) in [
                        (top_row, left_column),
                        (top_row, Some(column)),
                        (top_row, right_column),
                        (Some(row), left_column),
                        (Some(row), right_column),
                        (bottom_row, left_column),
                        (bottom_row, Some(column)),
                        (bottom_row, right_column),
                    ] {
                        if let (Some(row), Some(column)) = (row, column) {
                            adjacency_matrix_b[row][column] -= 1;
                        }
                    }
                }
            }
        }

        removed
    }

    #[inline(always)]
    fn gridfy_floor_plan(floor_plan: &[u8]) -> Vec<Vec<u8>> {
        floor_plan
            .split(|c| *c == b'\n')
            .filter_map(|row| {
                if !row.is_empty() {
                    Some(row.to_vec())
                } else {
                    None
                }
            })
            .collect()
    }

    fn build_adjacency_matrix(grid: &[Vec<u8>]) -> Vec<Vec<u32>> {
        let rows = grid.len();
        let columns = grid[0].len();
        let mut adjacency_matrix = vec![vec![0; columns]; rows];

        for (row_num, row) in grid.iter().enumerate() {
            for (column_num, _) in row.iter().enumerate().filter(|(_, c)| **c == PAPER_ROLL) {
                let left_column = column_num.checked_sub(1);
                let right_column = (column_num + 1 < columns).then_some(column_num + 1);
                let top_row = row_num.checked_sub(1);
                let bottom_row = (row_num + 1 < rows).then_some(row_num + 1);

                for (row, column) in [
                    (top_row, left_column),
                    (top_row, Some(column_num)),
                    (top_row, right_column),
                    (Some(row_num), left_column),
                    (Some(row_num), right_column),
                    (bottom_row, left_column),
                    (bottom_row, Some(column_num)),
                    (bottom_row, right_column),
                ] {
                    if let (Some(row), Some(column)) = (row, column) {
                        adjacency_matrix[row][column] += 1;
                    }
                }
            }
        }

        adjacency_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";

    #[test]
    fn test_accessible_paper_rolls() {
        assert_eq!(FloorPlan::accessible_paper_rolls(INPUT), 13);
    }

    #[test]
    fn test_removable_paper_rolls() {
        assert_eq!(FloorPlan::removable_paper_rolls(INPUT), 43);
    }
}
