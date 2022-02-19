use std::io::BufRead;

pub fn solve() {
    let result = solve_inner(std::io::stdin().lock().lines().map(|s| s.unwrap()));
    println!("{}", result.unwrap());
}

fn solve_inner<'a, Iter, T>(iter: Iter) -> Result<usize, String>
where
    Iter: Iterator<Item = T> + 'a,
    T: AsRef<str>,
{
    // bit counters
    let mut bc: Vec<i32> = Vec::new();
    for item in make_iterator(iter) {
        let item = match item {
            Ok(x) => x,
            Err(x) => return Err(x),
        };

        if bc.len() < item.len() {
            bc.resize(item.len(), 0);
        }

        for (i, b) in item.iter().enumerate() {
            match *b {
                1 => bc[i] += 1,
                0 => bc[i] -= 1,
                _ => {}
            }
        }
    }

    let mut gamma = 0_usize;
    let mut epsilon = 0_usize;

    for (i, cnt) in bc.iter().rev().enumerate() {
        let (dg, de) = if cnt.is_positive() { (1, 0) } else { (0, 1) };

        gamma += dg << i;
        epsilon += de << i;
    }

    Ok(gamma * epsilon)
}

pub fn make_iterator<'a, T, I>(iter: I) -> impl Iterator<Item = Result<Vec<u8>, String>> + 'a
where
    I: Iterator<Item = T> + 'a,
    T: AsRef<str>,
{
    iter.map(|s| {
        s.as_ref()
            .chars()
            .map(|c| match c {
                '1' => Ok(1_u8),
                '0' => Ok(0_u8),
                _ => Err("invalid character".to_string()),
            })
            .collect()
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_iterator() {
        let src = "010101\n101";
        let result: Result<Vec<_>, _> = make_iterator(src.lines()).collect();

        assert_eq!(result, Ok(vec![vec![0, 1, 0, 1, 0, 1], vec![1, 0, 1],]));

        let src = "012";
        let result: Result<Vec<_>, _> = make_iterator(src.lines()).collect();
        assert_eq!(result, Err("invalid character".to_string()));
    }

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

        let ans = solve_inner(src.lines()).unwrap();
        assert_eq!(ans, 198);
    }
}
