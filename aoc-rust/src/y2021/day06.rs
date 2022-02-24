use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

pub fn solve() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let simulator = src.parse::<Simulator>().unwrap();
    println!("{}", simulator.fish_num(80));
}

pub fn solve_part2() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let simulator = src.parse::<Simulator>().unwrap();
    println!("{}", simulator.fish_num(256));
}

#[allow(unused)]
fn the_number_of_fish(timer: u64, days: u64) -> u64 {
    let mut memo = HashMap::new();
    the_number_of_fish_memo(timer, days, &mut memo)
}

fn the_number_of_fish_memo(timer: u64, days: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if let Some(ans) = memo.get(&(timer, days)) {
        return *ans;
    }

    if days == 0 {
        let ans = 1;
        memo.insert((timer, days), ans);
        return ans;
    }

    if timer == 0 {
        let ans1 = the_number_of_fish_memo(6, days - 1, memo);
        memo.insert((timer, days), ans1);

        let ans2 = the_number_of_fish_memo(8, days - 1, memo);
        memo.insert((timer, days), ans2);
        return ans1 + ans2;
    }

    let ans = the_number_of_fish_memo(timer - 1, days - 1, memo);
    memo.insert((timer, days), ans);
    return ans;
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Simulator {
    fish: Vec<u64>,
}

impl Simulator {
    fn fish_num(&self, days: u64) -> u64 {
        let mut memo = HashMap::new();
        let mut total = 0;

        for f in self.fish.iter() {
            total += the_number_of_fish_memo(*f, days, &mut memo);
        }

        total
    }
}

impl FromStr for Simulator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fish = s
            .split(',')
            .map(|s| s.trim().parse::<u64>().map_err(|e| format!("{}", e).to_string()))
            .collect::<Result<_, _>>()?;
        Ok(Simulator { fish })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simulator_from_str() {
        let src = "3,4,3,1,2";
        let simulator = src.parse::<Simulator>().unwrap();

        assert_eq!(
            simulator,
            Simulator {
                fish: vec![3, 4, 3, 1, 2],
            }
        );
    }

    #[test]
    fn test_the_number_of_fish() {
        /*
         * 0: 3
         * 1: 2
         * 2: 1
         * 3: 0
         * 4: 6, 8
         * 5: 5, 7
         * 6: 4, 6
         * 7: 3, 5
         * 8: 2, 4
         * 9: 1, 3
         * 10: 0, 2
         * 11: 6, 1, 8
         * 12: 5, 0, 7
         * 13: 4, 6, 6, 8
         */
        let cases = vec![
            (3, 0, 1),
            (3, 3, 1),
            (3, 4, 2),
            (3, 10, 2),
            (3, 11, 3),
            (3, 12, 3),
            (3, 13, 4),
        ];

        for (n, days, ans) in cases.into_iter() {
            assert_eq!(
                the_number_of_fish(n, days),
                ans,
                "the_number_of_fish({}, {}) != {}",
                n,
                days,
                ans
            );
        }
    }

    #[test]
    fn example_case() {
        let src = "3,4,3,1,2";
        let simulator = src.parse::<Simulator>().unwrap();
        assert_eq!(26, simulator.fish_num(18));
        assert_eq!(5934, simulator.fish_num(80));
    }

    #[test]
    fn example_case_part2() {
        let src = "3,4,3,1,2";
        let simulator = src.parse::<Simulator>().unwrap();

        assert_eq!(26984457539, simulator.fish_num(256));
    }
}
