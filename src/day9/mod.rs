use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    x: u32,
    y: u32,
}

impl Tile {
    pub fn read_tiles(input: &str) -> Vec<Tile> {
        input
            .lines()
            .map(|line| {
                let Some((l, r)) = line.split_once(",") else {
                    unreachable!("Input must be well formatted.");
                };

                Tile {
                    x: l.parse().expect("Must be an u32"),
                    y: r.parse().expect("Must be an u32"),
                }
            })
            .collect()
    }

    pub fn area(&self, other: &Self) -> u64 {
        u64::from(self.x.abs_diff(other.x).add(1)) * u64::from(self.y.abs_diff(other.y).add(1))
    }

    pub fn largest_rectangle(tiles: &[Tile]) -> u64 {
        std::iter::successors(Some(tiles), |prev| {
            Some(&prev[1..]).filter(|slice| !slice.is_empty())
        })
        .flat_map(|tiles| {
            let head = &tiles[0];
            tiles[1..].iter().map(move |other| head.area(other))
        })
        .max()
        .unwrap_or(0)
    }

    pub fn largest_red_green_rectangle(tiles: &[Tile]) -> u64 {
        let wrapping_tiles = tiles
            .iter()
            .cycle()
            .take(tiles.len() + 1)
            .collect::<Vec<_>>();

        let mut horizontal_edges = wrapping_tiles
            .windows(2)
            .filter_map(|window| {
                if window[0].y == window[1].y {
                    Some((window[0].y, window[0], window[1]))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        horizontal_edges.sort_by_cached_key(|(index, _, _)| *index);

        let mut vertical_edges = wrapping_tiles
            .windows(2)
            .filter_map(|window| {
                if window[0].x == window[1].x {
                    Some((window[0].x, window[0], window[1]))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        vertical_edges.sort_by_cached_key(|(index, _, _)| *index);

        std::iter::successors(Some(tiles), |prev| {
            Some(&prev[1..]).filter(|slice| !slice.is_empty())
        })
        .flat_map(|tiles| {
            let head = &tiles[0];
            tiles[1..]
                .iter()
                .map(move |other| (head, other, head.area(other)))
        })
        .filter_map(|(l, r, area)| {
            if l.x == r.x || l.y == r.y {
                Some(area)
            } else {
                // dbg!((l, r, area));
                let min_x = l.x.min(r.x);
                let max_x = l.x.max(r.x);
                let min_y = l.y.min(r.y);
                let max_y = l.y.max(r.y);

                let top_edge = (Tile { x: l.x, y: min_y }, Tile { x: r.x, y: min_y });
                let top_edge_intersection = {
                    let (hrect_l, hrect_r) = &top_edge;
                    if hrect_l.x.abs_diff(hrect_r.x) == 1 {
                        false
                    } else {
                        let left_x = hrect_l.x.min(hrect_r.x);
                        let right_x = hrect_l.x.max(hrect_r.x);
                        // dbg!((&hrect_l, &hrect_r,));
                        let min = match vertical_edges
                            .binary_search_by_key(&left_x, |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };
                        let max = match vertical_edges
                            .binary_search_by_key(&right_x.add(1), |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };

                        vertical_edges[min..max]
                            .iter()
                            .skip_while(|(index, _, _)| *index == left_x)
                            .filter(|(_, vedge_l, vedge_r)| {
                                vedge_l != &hrect_l
                                    && vedge_l != &hrect_r
                                    && vedge_r != &hrect_l
                                    && vedge_r != &hrect_r
                            })
                            .any(|(_, vedge_l, vedge_r)| {
                                let min = vedge_l.y.min(vedge_r.y);
                                let max = vedge_l.y.max(vedge_r.y);
                                // dbg!((&vedge_l, &vedge_r, min <= hrect_l.y, hrect_l.y < max));
                                min <= hrect_l.y && hrect_l.y < max
                            })
                    }
                };
                if top_edge_intersection {
                    return None;
                }

                let bottom_edge = (Tile { x: l.x, y: max_y }, Tile { x: r.x, y: max_y });
                let bottom_edge_intersection = {
                    let (hrect_l, hrect_r) = &bottom_edge;
                    if hrect_l.x.abs_diff(hrect_r.x) == 1 {
                        false
                    } else {
                        let left_x = hrect_l.x.min(hrect_r.x);
                        let right_x = hrect_l.x.max(hrect_r.x);
                        // dbg!((&hrect_l, &hrect_r,));
                        let min = match vertical_edges
                            .binary_search_by_key(&left_x.sub(1), |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };
                        let max = match vertical_edges
                            .binary_search_by_key(&right_x, |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };

                        vertical_edges[min..max]
                            .iter()
                            .skip_while(|(index, _, _)| *index < left_x)
                            .filter(|(_, vedge_l, vedge_r)| {
                                vedge_l != &hrect_l
                                    && vedge_l != &hrect_r
                                    && vedge_r != &hrect_l
                                    && vedge_r != &hrect_r
                            })
                            .any(|(_, vedge_l, vedge_r)| {
                                let min = vedge_l.y.min(vedge_r.y);
                                let max = vedge_l.y.max(vedge_r.y);
                                // dbg!((&vedge_l, &vedge_r, min < hrect_l.y, hrect_l.y <= max));
                                min < hrect_l.y && hrect_l.y <= max
                            })
                    }
                };
                if bottom_edge_intersection {
                    return None;
                }

                let left_edge = (Tile { x: min_x, y: l.y }, Tile { x: min_x, y: r.y });
                let left_edge_intersection = {
                    let (vrect_l, vrect_r) = &left_edge;
                    if vrect_l.y.abs_diff(vrect_r.y) == 1 {
                        false
                    } else {
                        let top_y = vrect_l.y.min(vrect_r.y);
                        let bottom_y = vrect_l.y.max(vrect_r.y);
                        // dbg!((&vrect_l, &vrect_r,));
                        let min = match horizontal_edges
                            .binary_search_by_key(&top_y, |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };
                        let max = match horizontal_edges
                            .binary_search_by_key(&bottom_y.add(1), |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };

                        horizontal_edges[min..max]
                            .iter()
                            .skip_while(|(index, _, _)| *index == top_y)
                            .filter(|(_, hedge_l, hedge_r)| {
                                hedge_l != &vrect_l
                                    && hedge_l != &vrect_r
                                    && hedge_r != &vrect_l
                                    && hedge_r != &vrect_r
                            })
                            .any(|(_, hedge_l, hedge_r)| {
                                let min = hedge_l.x.min(hedge_r.x);
                                let max = hedge_l.x.max(hedge_r.x);
                                // dbg!((&hedge_l, &hedge_r, min <= vrect_l.x, vrect_l.x < max));
                                min <= vrect_l.x && vrect_l.x < max
                            })
                    }
                };
                if left_edge_intersection {
                    return None;
                }

                let right_edge = (Tile { x: max_x, y: l.y }, Tile { x: max_x, y: r.y });
                let right_edge_intersection = {
                    let (vrect_l, vrect_r) = &right_edge;
                    if vrect_l.y.abs_diff(vrect_r.y) == 1 {
                        false
                    } else {
                        let top_y = vrect_l.y.min(vrect_r.y);
                        let bottom_y = vrect_l.y.max(vrect_r.y);
                        // dbg!((&vrect_l, &vrect_r,));
                        let min = match horizontal_edges
                            .binary_search_by_key(&top_y.sub(1), |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };
                        let max = match horizontal_edges
                            .binary_search_by_key(&bottom_y, |(index, _, _)| *index)
                        {
                            Ok(index) => index,
                            Err(index) => index,
                        };

                        horizontal_edges[min..max]
                            .iter()
                            .skip_while(|(index, _, _)| *index < top_y)
                            .filter(|(_, hedge_l, hedge_r)| {
                                hedge_l != &vrect_l
                                    && hedge_l != &vrect_r
                                    && hedge_r != &vrect_l
                                    && hedge_r != &vrect_r
                            })
                            .any(|(_, hedge_l, hedge_r)| {
                                let min = hedge_l.x.min(hedge_r.x);
                                let max = hedge_l.x.max(hedge_r.x);
                                // dbg!((&hedge_l, &hedge_r, min < vrect_l.x, vrect_l.x <= max));
                                min < vrect_l.x && vrect_l.x <= max
                            })
                    }
                };
                if right_edge_intersection {
                    return None;
                }

                Some(area)
            }
        })
        .max()
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";

    const INPUT2: &str = "1,1
11,1
11,7
9,7
9,9
7,9
7,7
1,7
";

    const INPUT3: &str = "1,1
11,1
11,7
3,7
3,3
2,3
2,7
1,7
";

    #[test]
    fn test_largest_rectangle() {
        let tiles = Tile::read_tiles(INPUT);
        assert_eq!(Tile::largest_rectangle(&tiles), 50);
    }

    #[test]
    fn test_largest_red_green_rectangle() {
        let tiles = Tile::read_tiles(INPUT);
        assert_eq!(Tile::largest_red_green_rectangle(&tiles), 24);
        let tiles = Tile::read_tiles(INPUT2);
        assert_eq!(Tile::largest_red_green_rectangle(&tiles), 77);
        let tiles = Tile::read_tiles(INPUT3);
        assert_eq!(Tile::largest_red_green_rectangle(&tiles), 63);
    }
}
