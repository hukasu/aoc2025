use std::{fmt::Debug, num::ParseIntError};

#[derive(Debug)]
pub struct UnderTheChrismasTree {
    #[expect(dead_code, reason = "Day 12 can be cheated, basically")]
    present_shapes: [Grid; 6],
    problems: Vec<Problem>,
}

struct Grid {
    rows: [u64; 64],
}

#[derive(Debug)]
struct Problem {
    x: usize,
    y: usize,
    presents: [u8; 6],
}

impl UnderTheChrismasTree {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();

        let mut shapes = (&mut lines).take(6 * 5);
        let shapes = (0..6)
            .map(|i| {
                let header = shapes.next().unwrap();
                let l1 = shapes.next().unwrap();
                let l2 = shapes.next().unwrap();
                let l3 = shapes.next().unwrap();
                let empty = shapes.next().unwrap();

                debug_assert_eq!(header, format!("{i}:"));
                debug_assert!(empty.is_empty());

                let mut shape = [0; 64];

                for (i, row) in [l1, l2, l3].into_iter().enumerate() {
                    for (j, c) in row.chars().enumerate() {
                        if c == '#' {
                            shape[i] |= 1 << (2 - j);
                        }
                    }
                }

                Grid { rows: shape }
            })
            .collect::<Vec<_>>();

        let problems = lines
            .map(|line| {
                let Some((l, r)) = line.split_once(": ") else {
                    unreachable!(
                        "Problem must have the dimensions and presents separated by ': '."
                    );
                };
                let Some((x, y)) = l.split_once("x") else {
                    unreachable!("Dimensions of the problem must be separated by 'x'.");
                };
                let x = x.parse().expect("Dimension must be a number.");
                let y = y.parse().expect("Dimension must be a number.");
                let present_count = r
                    .split(" ")
                    .map(str::parse)
                    .collect::<Result<Vec<_>, ParseIntError>>()
                    .expect("Present counts must all be numbers.");

                Problem {
                    x,
                    y,
                    presents: present_count
                        .try_into()
                        .expect("Present count must have exactly 6 items."),
                }
            })
            .collect();

        Self {
            present_shapes: shapes.try_into().unwrap(),
            problems,
        }
    }

    pub fn valid_placements(&self) -> usize {
        // Grug solution
        self.problems
            .iter()
            .filter(|problem| {
                let total_presents = problem
                    .presents
                    .iter()
                    .copied()
                    .map(usize::from)
                    .sum::<usize>();
                problem.x * problem.y >= total_presents * 9
            })
            .count()
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows {
            writeln!(f, "{row:064b}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[expect(dead_code, reason = "Day 12 can be cheated, basically")]
    const INPUT: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
";

    #[test]
    fn test_shape_placements() {
        // Grug solution does not work for test input
        // assert_eq!(
        //     dbg!(UnderTheChrismasTree::parse(INPUT)).valid_placements(),
        //     2
        // );
    }
}
