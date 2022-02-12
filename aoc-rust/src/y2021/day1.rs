use std::io::BufRead;

pub fn solve() {
    println!(
        "{}",
        solve_inner(
            std::io::stdin()
                .lock()
                .lines()
                .flat_map(|s| s.unwrap().parse::<usize>().ok())
        )
    );
}

fn solve_inner<'a, T>(input: T) -> usize
where
    T: Iterator<Item = usize> + 'a,
{
    input
        .fold((0, None), |(cnt, prev), x| {
            if let Some(prev) = prev {
                (if prev < x { cnt + 1 } else { cnt }, Some(x))
            } else {
                (cnt, Some(x))
            }
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let src = r#"199
200
208
210
200
207
240
269
260
263"#;

        assert_eq!(
            7,
            solve_inner(src.lines().flat_map(|s| s.parse::<usize>().ok()))
        );
    }
}
