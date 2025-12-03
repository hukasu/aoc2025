use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub struct RangeChecker {
    cur: u64,
    end: u64,
    multiple: u32,
}

impl RangeChecker {
    fn new(mut start: u64, end: u64, multiple: u32) -> Self {
        let mut start_digits = start.ilog10() + 1;
        if !start_digits.is_multiple_of(multiple) {
            start_digits = start_digits.next_multiple_of(multiple);
            start = 10u64.pow(start_digits - 1);
        }

        let power_multiple = start_digits / multiple;
        let start_power = start_digits - power_multiple;
        let mut current_power = start_power;
        let mut cur = 0;

        loop {
            cur += (start / 10u64.pow(start_power)) * 10u64.pow(current_power);
            if current_power == 0 {
                break;
            }
            current_power -= power_multiple;
        }

        let mut tmp = Self { cur, end, multiple };
        while tmp.cur < start {
            tmp.next();
        }
        tmp
    }

    /// Returns the sum of the invalid IDs in ranges
    pub fn check_ranges(input: &str) -> u64 {
        let Some(first_line) = input.lines().next() else {
            unreachable!("Input will contain at least one line.");
        };
        first_line
            .split(",")
            .fold(HashSet::<u64>::new(), |mut set, range| {
                let Some((start, end)) = range.split_once("-").and_then(|(lhs, rhs)| {
                    lhs.parse::<u64>()
                        .and_then(|lhs| rhs.parse::<u64>().map(|rhs| (lhs, rhs)))
                        .ok()
                }) else {
                    unimplemented!("Input will be well formatted.");
                };
                Self::check_range(start, end, 2, &mut set);
                set
            })
            .into_iter()
            .sum()
    }

    /// Returns the sum of the invalid IDs in ranges
    /// taking into account IDs with multiple repetitions
    pub fn check_ranges_extra(input: &str) -> u64 {
        let Some(first_line) = input.lines().next() else {
            unreachable!("Input will contain at least one line.");
        };
        first_line
            .split(",")
            .fold(HashSet::<u64>::new(), |mut set, range| {
                let Some((start, end)) = range.split_once("-").and_then(|(lhs, rhs)| {
                    lhs.parse::<u64>()
                        .and_then(|lhs| rhs.parse::<u64>().map(|rhs| (lhs, rhs)))
                        .ok()
                }) else {
                    unimplemented!("Input will be well formatted.");
                };
                let digit_cout_of_end = end.ilog10() + 1;
                for i in 2..=digit_cout_of_end {
                    Self::check_range(start, end, i, &mut set);
                }
                set
            })
            .into_iter()
            .sum()
    }

    fn check_range(start: u64, end: u64, multiple: u32, accumulator: &mut HashSet<u64>) {
        accumulator.extend(RangeChecker::new(start, end, multiple));
    }
}

impl Iterator for RangeChecker {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur > self.end {
            None
        } else {
            let res = self.cur;

            let start_digits = self.cur.ilog10() + 1;
            let power_multiple = start_digits / self.multiple;
            let start_power = start_digits - power_multiple;
            let mut current_power = start_power;

            loop {
                self.cur += 10u64.pow(current_power);
                if current_power == 0 {
                    break;
                }
                current_power -= power_multiple;
            }

            let after_sum_digits = self.cur.ilog10() + 1;
            if !after_sum_digits.is_multiple_of(self.multiple) {
                *self = Self::new(self.cur, self.end, self.multiple);
            }

            Some(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_range {
        ($start:literal, $end:literal, $multiple:literal) => {{
            let mut hash_set = HashSet::new();
            RangeChecker::check_range($start, $end, $multiple, &mut hash_set);
            let mut v = Vec::from_iter(hash_set);
            v.sort();
            v
        }};
    }

    #[test]
    fn test_new_range_checker() {
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 2),
            RangeChecker {
                cur: 10001000,
                end: 99999999,
                multiple: 2,
            }
        );
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 3),
            RangeChecker {
                cur: 100100100,
                end: 99999999,
                multiple: 3,
            }
        );
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 4),
            RangeChecker {
                cur: 10101010,
                end: 99999999,
                multiple: 4,
            }
        );
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 5),
            RangeChecker {
                cur: 1010101010,
                end: 99999999,
                multiple: 5,
            }
        );
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 6),
            RangeChecker {
                cur: 101010101010,
                end: 99999999,
                multiple: 6,
            }
        );
        assert_eq!(
            RangeChecker::new(1000000, 99999999, 7),
            RangeChecker {
                cur: 1111111,
                end: 99999999,
                multiple: 7,
            }
        );
    }

    #[test]
    fn test_ranges() {
        assert_eq!(test_range!(11, 22, 2), &[11, 22]);
        assert_eq!(test_range!(95, 115, 2), &[99]);
        assert_eq!(test_range!(998, 1012, 2), &[1010]);
        assert_eq!(test_range!(1188511880, 1188511890, 2), &[1188511885]);
        assert_eq!(test_range!(222220, 222224, 2), &[222222]);
        assert_eq!(test_range!(1698522, 1698528, 2), &[]);
        assert_eq!(test_range!(446443, 446449, 2), &[446446]);
        assert_eq!(test_range!(38593856, 38593862, 2), &[38593859]);
        assert_eq!(test_range!(565653, 565659, 2), &[]);
        assert_eq!(test_range!(824824821, 824824827, 2), &[]);
        assert_eq!(test_range!(2121212118, 2121212124, 2), &[]);
    }

    #[test]
    fn test_ranges_extra() {
        assert_eq!(test_range!(95, 115, 3), &[111]);
        assert_eq!(test_range!(998, 1012, 3), &[999]);
        assert_eq!(test_range!(565653, 565659, 3), &[565656]);
        assert_eq!(test_range!(824824821, 824824827, 3), &[824824824]);
        assert_eq!(test_range!(2121212118, 2121212124, 5), &[2121212121]);
    }

    #[test]
    fn test_ranges_file() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(RangeChecker::check_ranges(input), 1227775554);
    }

    #[test]
    fn test_ranges_extra_file() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(RangeChecker::check_ranges_extra(input), 4174379265);
    }
}
