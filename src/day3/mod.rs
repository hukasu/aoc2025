use std::ops::{Add, Sub};

pub struct PowerBank;

impl PowerBank {
    /// Takes in multiple banks separated by `\n`
    pub fn best_joltage_multiple_banks(storage: &str, batteries_to_turn_on: usize) -> u64 {
        storage
            .lines()
            .map(|line| Self::best_joltage(line, batteries_to_turn_on))
            .sum()
    }

    /// Takes in a single bank
    pub fn best_joltage(bank: &str, batteries_to_turn_on: usize) -> u64 {
        if bank.is_empty() {
            0
        } else {
            Self::joltage_dynamic_programing(bank, batteries_to_turn_on)
        }
    }

    fn joltage_dynamic_programing(bank: &str, batteries_to_turn_on: usize) -> u64 {
        if batteries_to_turn_on == 0 {
            return 0;
        }

        let relevant_batteries = &bank[..=bank.len().sub(batteries_to_turn_on)];
        let (max, index) =
            relevant_batteries
                .chars()
                .enumerate()
                .fold(
                    ('0', 0),
                    |(max, index), (i, char)| if char > max { (char, i) } else { (max, index) },
                );
        let max = max.to_digit(10).map(u64::from).expect("Must be a digit.");
        let next =
            Self::joltage_dynamic_programing(&bank[index.add(1)..], batteries_to_turn_on - 1);
        let multiplier =
            10u64.pow(u32::try_from(batteries_to_turn_on - 1).expect("Should never overflow"));

        max * multiplier + next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banks() {
        assert_eq!(PowerBank::best_joltage("987654321111111", 2), 98);
        assert_eq!(PowerBank::best_joltage("811111111111119", 2), 89);
        assert_eq!(PowerBank::best_joltage("234234234234278", 2), 78);
        assert_eq!(PowerBank::best_joltage("818181911112111", 2), 92);
    }

    #[test]
    fn test_multiple_banks() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(PowerBank::best_joltage_multiple_banks(input, 2), 357);
    }

    #[test]
    fn test_multiple_banks_12_digits() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(
            PowerBank::best_joltage_multiple_banks(input, 12),
            3121910778619
        );
    }
}
