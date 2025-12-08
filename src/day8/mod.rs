use std::{cmp::Reverse, num::ParseIntError};

pub struct JunctionBox {
    x: f32,
    y: f32,
    z: f32,
}

impl JunctionBox {
    pub fn read_positions(position_list: &str) -> Vec<JunctionBox> {
        position_list
            .lines()
            .map(|line| {
                let Ok(line_split) = line
                    .split(",")
                    .map(|coord| coord.parse::<u32>())
                    .collect::<Result<Vec<u32>, ParseIntError>>()
                else {
                    unreachable!("Input must be well formatted.");
                };
                assert_eq!(line_split.len(), 3);
                JunctionBox {
                    x: line_split[0] as f32,
                    y: line_split[1] as f32,
                    z: line_split[2] as f32,
                }
            })
            .collect()
    }

    pub fn distance(&self, other: &Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn largest_circuits(junction_boxes: &[JunctionBox], connections: usize) -> u32 {
        let mut junction_box_circuit = vec![0; junction_boxes.len()];
        let mut circuits = vec![0; junction_boxes.len()];
        circuits[0] = junction_boxes.len() as u32;
        let mut next_circuit_id = 1;

        let mut distances = Self::pairwise_distances(junction_boxes);
        distances.sort_unstable_by(|(_, _, l), (_, _, r)| l.total_cmp(r));

        for (l, r, _) in distances.into_iter().take(connections) {
            match (junction_box_circuit[l], junction_box_circuit[r]) {
                (0, 0) => {
                    circuits[0] -= 1;
                    circuits[0] -= 1;
                    junction_box_circuit[l] = next_circuit_id;
                    junction_box_circuit[r] = next_circuit_id;
                    circuits[next_circuit_id] += 1;
                    circuits[next_circuit_id] += 1;
                    next_circuit_id += 1;
                }
                (0, jbc_r) => {
                    circuits[0] -= 1;
                    junction_box_circuit[l] = jbc_r;
                    circuits[jbc_r] += 1;
                }
                (jbc_l, 0) => {
                    circuits[0] -= 1;
                    junction_box_circuit[r] = jbc_l;
                    circuits[jbc_l] += 1;
                }
                (jbc_l, jbc_r) => {
                    for jbc in junction_box_circuit.iter_mut() {
                        if *jbc == jbc_r {
                            *jbc = jbc_l;
                        }
                    }
                    let old_count = circuits[jbc_r];
                    circuits[jbc_r] = 0;
                    circuits[jbc_l] += old_count;
                }
            }
        }

        circuits[1..].sort_unstable_by_key(|count| Reverse(*count));

        circuits[1..4].iter().product()
    }

    pub fn distance_to_wall(junction_boxes: &[JunctionBox]) -> u64 {
        let len = junction_boxes.len() as u32;
        let mut junction_box_circuit = vec![0; junction_boxes.len()];
        let mut circuits = vec![0; junction_boxes.len()];
        circuits[0] = len;
        let mut next_circuit_id = 1;

        let mut distances = Self::pairwise_distances(junction_boxes);
        distances.sort_unstable_by(|(_, _, l), (_, _, r)| l.total_cmp(r));

        let mut cache_l = 0;
        let mut cache_r = 0;

        for (l, r, _) in distances.into_iter() {
            cache_l = l;
            cache_r = r;

            match (junction_box_circuit[l], junction_box_circuit[r]) {
                (0, 0) => {
                    circuits[0] -= 1;
                    circuits[0] -= 1;
                    junction_box_circuit[l] = next_circuit_id;
                    junction_box_circuit[r] = next_circuit_id;
                    circuits[next_circuit_id] += 1;
                    circuits[next_circuit_id] += 1;
                    next_circuit_id += 1;
                }
                (0, jbc_r) => {
                    circuits[0] -= 1;
                    junction_box_circuit[l] = jbc_r;
                    circuits[jbc_r] += 1;
                }
                (jbc_l, 0) => {
                    circuits[0] -= 1;
                    junction_box_circuit[r] = jbc_l;
                    circuits[jbc_l] += 1;
                }
                (jbc_l, jbc_r) => {
                    let receiver = jbc_l.min(jbc_r);
                    let donator = jbc_l.max(jbc_r);

                    for jbc in junction_box_circuit.iter_mut() {
                        if *jbc == donator {
                            *jbc = receiver;
                        }
                    }
                    let old_count = circuits[donator];
                    circuits[donator] = 0;
                    circuits[receiver] += old_count;
                }
            }

            if circuits[1] == len {
                break;
            }
        }

        junction_boxes[cache_l].x as u64 * junction_boxes[cache_r].x as u64
    }

    fn pairwise_distances(junction_boxes: &[JunctionBox]) -> Vec<(usize, usize, f32)> {
        std::iter::successors(Some(junction_boxes), |head| {
            Some(&head[1..]).filter(|sub_list| !sub_list.is_empty())
        })
        .enumerate()
        .flat_map(|(i, sub_list)| {
            let head = &sub_list[0];
            sub_list[1..]
                .iter()
                .enumerate()
                .map(move |(j, other)| (i, i + 1 + j, head.distance(other)))
        })
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";

    #[test]
    fn test_largest_circuits() {
        let junction_boxes = JunctionBox::read_positions(INPUT);
        assert_eq!(JunctionBox::largest_circuits(&junction_boxes, 10), 40);
    }

    #[test]
    fn test_distance_to_wall() {
        let junction_boxes = JunctionBox::read_positions(INPUT);
        assert_eq!(JunctionBox::distance_to_wall(&junction_boxes), 25272);
    }
}
