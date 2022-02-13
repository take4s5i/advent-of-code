use std::{io::BufRead, str::FromStr};

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
    let (depth, pos): (usize, usize) = iter.fold((0, 0), |(depth, pos), cmd| match cmd {
        Command::Forward(x) => (depth, pos + x),
        Command::Up(x) => (depth - x, pos),
        Command::Down(x) => (depth + x, pos),
    });

    depth * pos
}

#[derive(PartialEq, Eq, Debug)]
pub enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl Command {
    pub fn set_amount(&mut self, v: usize) {
        match self {
            Command::Forward(_) => *self = Command::Forward(v),
            Command::Up(_) => *self = Command::Up(v),
            Command::Down(_) => *self = Command::Down(v),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CommandParseError {
    InvalidFormat,
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(' ');
        let mut cmd = iter
            .next()
            .ok_or(CommandParseError::InvalidFormat)
            .and_then(|s| match s.to_lowercase().as_str() {
                "forward" => Ok(Command::Forward(0)),
                "up" => Ok(Command::Up(0)),
                "down" => Ok(Command::Down(0)),
                _ => Err(CommandParseError::InvalidFormat),
            })?;

        let amount = iter
            .next()
            .ok_or(CommandParseError::InvalidFormat)
            .and_then(|s| {
                s.parse::<usize>()
                    .map_err(|_| CommandParseError::InvalidFormat)
            })?;

        if iter.next() != None {
            return Err(CommandParseError::InvalidFormat);
        }

        cmd.set_amount(amount);
        Ok(cmd)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn command_parse() {
        assert_eq!("forward 10".parse::<Command>(), Ok(Command::Forward(10)));
        assert_eq!("UP 20".parse::<Command>(), Ok(Command::Up(20)));
        assert_eq!("Down 30".parse::<Command>(), Ok(Command::Down(30)));
        assert_eq!(
            "forward 0 ".parse::<Command>(),
            Err(CommandParseError::InvalidFormat)
        );
        assert_eq!(
            " forward 0".parse::<Command>(),
            Err(CommandParseError::InvalidFormat)
        );
        assert_eq!(
            "forward -1".parse::<Command>(),
            Err(CommandParseError::InvalidFormat)
        );
        assert_eq!(
            "hoge 0".parse::<Command>(),
            Err(CommandParseError::InvalidFormat)
        );
    }

    #[test]
    fn solve() {
        let s = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        let ans = solve_inner(s.lines().map(|s| s.parse::<Command>().unwrap()));
        assert_eq!(150, ans);
    }
}
