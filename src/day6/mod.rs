use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Worksheet {
    operations: Vec<Operation>,
}

impl Worksheet {
    pub fn solve_worksheet(worksheet: &str) -> u64 {
        let worksheet = Self::parse(worksheet);
        worksheet
            .operations
            .iter()
            .map(|operation| operation.solve())
            .sum()
    }

    pub fn solve_cephalopodian_worksheet(worksheet: &str) -> u64 {
        let worksheet = Self::parse_cephalopodian(worksheet);
        worksheet
            .operations
            .iter()
            .map(|operation| operation.solve())
            .sum()
    }

    fn parse(worksheet: &str) -> Worksheet {
        let mut lines = worksheet.lines().rev();

        let Some(operators) = lines.next() else {
            unreachable!("Lines will never be empty.");
        };

        let mut number_rows = lines
            .map(|row| {
                row.split(" ")
                    .filter_map(|number| number.parse::<u64>().ok())
            })
            .collect::<Vec<_>>();

        let mut operations = vec![];

        for operator in operators.split(" ").filter_map(|operator| {
            if operator.is_empty() {
                None
            } else {
                match operator.trim() {
                    "+" => Some(Operator::Sum),
                    "*" => Some(Operator::Multiplication),
                    _ => unreachable!("Operator input must be well formatted."),
                }
            }
        }) {
            let Some(numbers) = number_rows
                .iter_mut()
                .map(|number| number.next())
                .collect::<Option<Vec<u64>>>()
            else {
                unreachable!("All rows must have the same number of items as the operators row.");
            };
            operations.push(Operation { numbers, operator });
        }

        Worksheet { operations }
    }

    fn parse_cephalopodian(worksheet: &str) -> Worksheet {
        let mut lines = worksheet.lines().rev();

        let Some(operators) = lines.next() else {
            unreachable!("Lines will never be empty.");
        };

        let mut number_rows = lines.map(|line| line.chars().rev()).collect::<Vec<_>>();
        number_rows.reverse();

        let mut operations = vec![];
        let mut number_cache = vec![];

        let mut next_is_skip = false;
        for operator in operators.chars().rev() {
            if next_is_skip {
                for row in number_rows.iter_mut() {
                    let _ = row.next();
                }
                next_is_skip = false;
            } else {
                let mut number = 0;
                for row in number_rows.iter_mut() {
                    let Some(digit) = row.next() else {
                        unreachable!(
                            "All rows must have the same number of characters as the operators."
                        );
                    };
                    if digit != ' ' {
                        number = number * 10 + u64::from(digit.to_digit(10).expect("msg"))
                    }
                }
                number_cache.push(number);

                match operator {
                    '+' => {
                        operations.push(Operation {
                            numbers: std::mem::take(&mut number_cache),
                            operator: Operator::Sum,
                        });
                        next_is_skip = true;
                    }
                    '*' => {
                        operations.push(Operation {
                            numbers: std::mem::take(&mut number_cache),
                            operator: Operator::Multiplication,
                        });
                        next_is_skip = true;
                    }
                    ' ' => (),
                    _ => unreachable!("Input must be well formatted."),
                }
            }
        }

        Worksheet { operations }
    }
}

#[derive(Debug)]
struct Operation {
    numbers: Vec<u64>,
    operator: Operator,
}

impl Operation {
    fn solve(&self) -> u64 {
        let func: fn(u64, u64) -> u64 = match self.operator {
            Operator::Sum => Add::add,
            Operator::Multiplication => Mul::mul,
        };
        self.numbers
            .iter()
            .copied()
            .reduce(func)
            .expect("Numbers will never be empty")
    }
}

#[derive(Debug)]
enum Operator {
    Sum,
    Multiplication,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";

    #[test]
    fn test_solver() {
        assert_eq!(Worksheet::solve_worksheet(INPUT), 4277556);
    }

    #[test]
    fn test_cephalopodian_solver() {
        assert_eq!(Worksheet::solve_cephalopodian_worksheet(INPUT), 3263827);
    }
}
