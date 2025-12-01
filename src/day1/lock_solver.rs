use crate::day1::Lock;

pub struct LockSolver;

impl LockSolver {
    pub fn find_password(instructions: &str) -> u32 {
        let mut counter = 0;
        let mut lock = Lock::default();

        for line in instructions.lines() {
            let dir = &line[..1];
            let Ok(moves) = line[1..].parse::<i32>() else {
                unreachable!("Input must be well formatted.");
            };

            match dir {
                "R" => {
                    lock = lock + moves;
                }
                "L" => {
                    lock = lock - moves;
                }
                _ => unreachable!("Input must be well formatted."),
            }

            if *lock == 0 {
                counter += 1;
            }
        }

        counter
    }

    #[expect(non_snake_case, reason = "Contains hex in name")]
    pub fn find_password_method_0x434C49434B(instructions: &str) -> u32 {
        let mut counter = 0;
        let mut lock = Lock::default();

        for line in instructions.lines() {
            let dir = &line[..1];
            let Ok(moves) = line[1..].parse::<i32>() else {
                unreachable!("Input must be well formatted.");
            };

            match dir {
                "R" => {
                    let (new_lock, turns) = lock.add_with_turns(moves);
                    lock = new_lock;
                    counter += turns;
                }
                "L" => {
                    let (new_lock, turns) = lock.sub_with_turns(moves);
                    lock = new_lock;
                    counter += turns;
                }
                _ => unreachable!("Input must be well formatted."),
            }
        }

        counter
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(LockSolver::find_password(input), 3);
    }

    #[test]
    fn test_input_2() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
        assert_eq!(LockSolver::find_password_method_0x434C49434B(input), 6);
    }
}
