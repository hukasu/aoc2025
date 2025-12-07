use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct TachyonManifold<'a> {
    /// Number of characters in each line of the manifold,
    /// does not account for the new line characters.
    width: usize,
    manifold: &'a [u8],
    starting_beam: usize,
}

impl<'a> TachyonManifold<'a> {
    pub fn new(manifold: &'a [u8]) -> TachyonManifold<'a> {
        let Some(first_line_break) = manifold.iter().position(|c| *c == b'\n') else {
            unreachable!("Input will always have multiple lines.");
        };
        let Some(starting_beam) = manifold[..first_line_break].iter().position(|c| *c == b'S')
        else {
            unreachable!("Input will always have a start on the first line.");
        };
        let width = first_line_break;

        TachyonManifold {
            width,
            manifold: &manifold[(first_line_break + 1)..],
            starting_beam,
        }
    }

    pub fn count_splits(&self) -> u64 {
        let TachyonManifold {
            width,
            manifold,
            starting_beam,
        } = self;

        let mut splits = 0;

        let mut beams = HashSet::new();
        beams.insert(*starting_beam);
        let mut beams_b = HashSet::new();

        for line in manifold.chunks(width + 1) {
            for beam in beams.drain() {
                if line[beam] == b'^' {
                    splits += 1;
                    if let Some(left) = beam.checked_sub(1) {
                        beams_b.insert(left);
                    }
                    if let Some(right) = Some(beam + 1).filter(|beam| beam < width) {
                        beams_b.insert(right);
                    }
                } else {
                    beams_b.insert(beam);
                }
            }
            std::mem::swap(&mut beams, &mut beams_b);
        }

        splits
    }

    pub fn count_timelines(&self) -> u64 {
        let TachyonManifold {
            width,
            manifold,
            starting_beam,
        } = self;

        let mut timelines = 1;

        let mut beams = HashMap::new();
        beams.insert(*starting_beam, 1);
        let mut beams_b = HashMap::new();

        for line in manifold.chunks(width + 1) {
            for (beam, count) in beams.drain() {
                if line[beam] == b'^' {
                    timelines += count;
                    if let Some(left) = beam.checked_sub(1) {
                        beams_b
                            .entry(left)
                            .and_modify(|entry_count: &mut u64| *entry_count += count)
                            .or_insert(count);
                    }
                    if let Some(right) = Some(beam + 1).filter(|beam| beam < width) {
                        beams_b
                            .entry(right)
                            .and_modify(|entry_count: &mut u64| *entry_count += count)
                            .or_insert(count);
                    }
                } else {
                    beams_b
                        .entry(beam)
                        .and_modify(|entry_count: &mut u64| *entry_count += count)
                        .or_insert(count);
                }
            }
            std::mem::swap(&mut beams, &mut beams_b);
        }

        timelines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = b".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_splits() {
        assert_eq!(TachyonManifold::new(INPUT).count_splits(), 21);
    }

    #[test]
    fn test_timelines() {
        assert_eq!(TachyonManifold::new(INPUT).count_timelines(), 40);
    }
}
