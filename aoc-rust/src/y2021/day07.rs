use std::io::Read;

pub fn solve() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let data: Vec<_> = src
        .split(",")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect();

    let calc = CostCalculator::new_with_liner_cost();
    println!("{}", calc.optimal_cost(&data));
}

pub fn solve_part2() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let data: Vec<_> = src
        .split(",")
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect();

    let calc = CostCalculator::new_with_accumulative_cost();
    println!("{}", calc.optimal_cost(&data));
}

struct CostCalculator {
    cost_func: fn(u64, u64) -> u64,
}

impl CostCalculator {
    fn new_with_liner_cost() -> CostCalculator {
        CostCalculator {
            cost_func: abs_diff as fn(u64, u64) -> u64,
        }
    }

    fn new_with_accumulative_cost() -> CostCalculator {
        CostCalculator {
            cost_func: acc_diff as fn(u64, u64) -> u64,
        }
    }

    fn optimal_cost(&self, data: &[u64]) -> u64 {
        let mut min = u64::MAX;

        for n in 0..data.len() {
            min = std::cmp::min(min, self.compute_cost(data, n.try_into().unwrap()));
        }

        min
    }

    fn compute_cost(&self, data: &[u64], step: u64) -> u64 {
        data.iter().map(|&x| (self.cost_func)(x, step)).sum()
    }
}

fn abs_diff(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn acc_diff(a: u64, b: u64) -> u64 {
    let d = abs_diff(a, b);

    if d % 2 == 0 {
        (1 + d) * d / 2
    } else {
        (d * (d - 1) / 2) + d
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compute_cost() {
        let data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let calc = CostCalculator::new_with_liner_cost();

        assert_eq!(41, calc.compute_cost(&data, 1));
        assert_eq!(37, calc.compute_cost(&data, 2));
        assert_eq!(39, calc.compute_cost(&data, 3));
        assert_eq!(71, calc.compute_cost(&data, 10));
    }

    #[test]
    fn example_case() {
        let data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let calc = CostCalculator::new_with_liner_cost();

        assert_eq!(37, calc.optimal_cost(&data));
    }

    #[test]
    fn example_case_part2() {
        let data = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        let calc = CostCalculator::new_with_accumulative_cost();

        assert_eq!(168, calc.optimal_cost(&data));
    }
}
