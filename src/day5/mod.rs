use std::ops::RangeInclusive;

pub struct Inventory;

impl Inventory {
    pub fn spoiled_items(ranges_and_items: &str) -> usize {
        let (mut ranges, items) = Self::extract_ranges_and_items(ranges_and_items);
        ranges.sort_by_cached_key(|range| *range.end());
        items
            .into_iter()
            .filter(|item| Self::is_fresh(*item, &ranges))
            .count()
    }

    pub fn fresh_items(ranges_and_items: &str) -> usize {
        let mut ranges = Self::extract_ranges_only(ranges_and_items);
        ranges.sort_by_cached_key(|range| *range.start());

        let mut iter = ranges.into_iter();
        let Some(mut head) = iter.next() else {
            unreachable!("Ranges will never be empty.");
        };
        let mut count = 0;

        for range in iter {
            if head.end() < range.start() {
                count += head.count();
                head = range
            } else {
                count += (*head.start()..*range.start()).count();
                head = *range.start()..=*(head.end().max(range.end()));
            }
        }
        count += head.count();

        count
    }

    fn extract_ranges_and_items(ranges_and_items: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
        let (ranges, items, _) = ranges_and_items.lines().fold(
            (vec![], vec![], false),
            |(mut ranges, mut items, mid_point), line| {
                if line.is_empty() {
                    (ranges, items, true)
                } else if mid_point {
                    let Ok(item) = line.parse::<u64>() else {
                        unreachable!("Input must only have u64 items.");
                    };

                    items.push(item);

                    (ranges, items, mid_point)
                } else {
                    let Some((l, r)) = line.split_once("-") else {
                        unreachable!("Input must be well formatted.");
                    };
                    let Ok(start) = l.parse::<u64>() else {
                        unreachable!("Input must only have u64 ranges.");
                    };
                    let Ok(end) = r.parse::<u64>() else {
                        unreachable!("Input must only have u64 ranges.");
                    };

                    ranges.push(start..=end);

                    (ranges, items, mid_point)
                }
            },
        );

        (ranges, items)
    }

    fn extract_ranges_only(ranges_and_items: &str) -> Vec<RangeInclusive<u64>> {
        ranges_and_items
            .lines()
            .scan((), |_, line| {
                if line.is_empty() {
                    return None;
                }
                let Some((l, r)) = line.split_once("-") else {
                    unreachable!("Input must be well formatted.");
                };
                let Ok(start) = l.parse::<u64>() else {
                    unreachable!("Input must only have u64 ranges.");
                };
                let Ok(end) = r.parse::<u64>() else {
                    unreachable!("Input must only have u64 ranges.");
                };

                Some(start..=end)
            })
            .collect()
    }

    fn is_fresh(item: u64, ranges: &[RangeInclusive<u64>]) -> bool {
        ranges.iter().any(|range| range.contains(&item))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_spoiled() {
        assert_eq!(Inventory::spoiled_items(INPUT), 3);
    }

    #[test]
    fn test_fresh() {
        assert_eq!(Inventory::fresh_items(INPUT), 14);
    }
}
