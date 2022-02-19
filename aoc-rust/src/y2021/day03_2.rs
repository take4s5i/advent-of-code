use std::io::BufRead;

pub fn solve() {
    let raw: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|s| {
            Bits(
                s.unwrap()
                    .chars()
                    .map(|c| match c {
                        '0' => 0_u8,
                        '1' => 1_u8,
                        _ => panic!("unexpected char is in data"),
                    })
                    .collect::<Vec<u8>>(),
            )
        })
        .collect();

    let data = Data { raw };

    let ans = solve_inner(&data);
    println!("{}", ans);
}

fn solve_inner(data: &Data) -> u64 {
    let ogr = RatingFinder(RatingType::OxygenGenerator).find_rating(data);
    let csr = RatingFinder(RatingType::CO2Scrubber).find_rating(data);

    ogr.unwrap().to_u64() * csr.unwrap().to_u64()
}

enum RatingType {
    OxygenGenerator,
    CO2Scrubber,
}

struct Data {
    raw: Vec<Bits>,
}

#[derive(PartialEq, Eq, Clone)]
struct Bits(Vec<u8>);

impl Bits {
    fn nth(&self, n: usize) -> u8 {
        self.0[n]
    }

    fn to_u64(&self) -> u64 {
        self.0
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (i, x)| acc + ((*x as u64) << i))
    }
}

struct RatingFinder(RatingType);

impl RatingFinder {
    fn find_rating(&self, data: &Data) -> Option<Bits> {
        self.find_rating_inner(&data.raw, 0)
    }

    fn find_rating_inner(&self, data: &Vec<Bits>, pos: usize) -> Option<Bits> {
        match data.len() {
            0 => None,
            1 => Some(data[0].clone()),
            _ => {
                let (has1, has0): (Vec<_>, Vec<_>) =
                    data.iter().cloned().partition(|v| v.nth(pos) > 0);
                if has1.len() >= has0.len() {
                    let next = match self.0 {
                        RatingType::OxygenGenerator => has1,
                        RatingType::CO2Scrubber => has0,
                    };

                    return self.find_rating_inner(&next, pos + 1);
                } else {
                    let next = match self.0 {
                        RatingType::OxygenGenerator => has0,
                        RatingType::CO2Scrubber => has1,
                    };

                    return self.find_rating_inner(&next, pos + 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let src = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

        let raw: Vec<_> = src
            .lines()
            .map(|s| {
                Bits(
                    s.chars()
                        .map(|c| match c {
                            '0' => 0_u8,
                            '1' => 1_u8,
                            _ => panic!("unexpected char is in data"),
                        })
                        .collect::<Vec<u8>>(),
                )
            })
            .collect();

        let data = Data { raw };

        let ans = solve_inner(&data);
        assert_eq!(ans, 230);
    }
}
