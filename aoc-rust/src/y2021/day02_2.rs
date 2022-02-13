pub use super::day02::{Command, CommandParseError};
use std::io::BufRead;

pub fn solve() {
    println!(
        "{}",
        solve_inner(
            std::io::stdin()
                .lock()
                .lines()
                .map(|s| s.unwrap().parse::<Command>().unwrap())
        ),
    );
}

fn solve_inner<I>(iter: I) -> usize
where
    I: Iterator<Item = Command>,
{
    let (_, depth, pos): (usize, usize, usize) =
        iter.fold((0, 0, 0), |(aim, depth, pos), cmd| match cmd {
            Command::Forward(x) => (aim, depth + aim * x, pos + x),
            Command::Up(x) => (aim - x, depth, pos),
            Command::Down(x) => (aim + x, depth, pos),
        });

    depth * pos
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case1() {
        let src = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        let ans = solve_inner(src.lines().map(|s| s.parse::<Command>().unwrap()));
        assert_eq!(900, ans);
    }
}
