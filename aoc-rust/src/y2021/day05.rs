use std::collections::HashMap;
use std::io::Read;
use std::str::FromStr;

pub fn solve() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let field = src.parse::<BasicField>().unwrap();
    println!("{}", field.dengerous_points().count());
}

pub fn solve_part2() {
    let mut src = String::new();
    std::io::stdin().read_to_string(&mut src).unwrap();

    let field = src.parse::<DiagonalField>().unwrap();
    println!("{}", field.dengerous_points().count());
}

#[derive(PartialEq, Eq, Default, Clone, Debug, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn unit(&self) -> Point {
        let dx = self.x.cmp(&0) as i64;
        let dy = self.y.cmp(&0) as i64;

        Point { x: dx, y: dy }
    }
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, ',');
        let x = iter
            .next()
            .ok_or(format!("Point Parse Error: invalid format {}", s).to_string())
            .and_then(|s| {
                s.trim().parse::<i64>().map_err(|e| {
                    format!("Point Parse Error: cannot parse x. {}, {}", s, e).to_string()
                })
            })?;

        let y = iter
            .next()
            .ok_or(format!("Point Parse Error: invalid format {}", s).to_string())
            .and_then(|s| {
                s.trim().parse::<i64>().map_err(|e| {
                    format!("Point Parse Error: cannot parse y. {}, {}", s, e).to_string()
                })
            })?;

        Ok(Point { x, y })
    }
}

struct LineIterator {
    current: Point,
    delta: Point,
    len: i64,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len < 0 {
            return None;
        }

        let ret = self.current;
        self.current += self.delta;
        self.len -= 1;
        Some(ret)
    }
}

#[derive(PartialEq, Eq, Default, Clone, Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    #[allow(dead_code)]
    fn new(x1: i64, y1: i64, x2: i64, y2: i64) -> Line {
        Line {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        }
    }

    #[allow(dead_code)]
    fn iter(&self) -> LineIterator {
        self.iter_with_options(false)
    }

    fn iter_with_options(&self, considering_diagonals: bool) -> LineIterator {
        let current = self.start;
        let delta = self.end - self.start;
        let len = match (delta.x == 0, delta.y == 0) {
            (true, true) => -1,
            (true, false) => delta.y.abs(),
            (false, true) => delta.x.abs(),
            (false, false) => {
                if considering_diagonals && delta.y.abs() == delta.x.abs() {
                    delta.y.abs()
                } else {
                    -1
                }
            }
        };

        LineIterator {
            current,
            delta: delta.unit(),
            len,
        }
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(2, "->");
        let start = iter
            .next()
            .ok_or("Line Parse Error: invalid format".to_string())
            .and_then(|s| s.parse::<Point>())?;

        let end = iter
            .next()
            .ok_or("Line Parse Error: invalid format".to_string())
            .and_then(|s| s.parse::<Point>())?;

        Ok(Line { start, end })
    }
}

type BasicField = Field<false>;
type DiagonalField = Field<true>;

#[derive(PartialEq, Eq, Default, Clone, Debug)]
struct Field<const C: bool> {
    lines: Vec<Line>,
    considering_diagonals: bool,
}

impl<const C: bool> Field<C> {
    #[allow(dead_code)]
    fn new(lines: Vec<Line>) -> Field<C> {
        Field {
            lines,
            considering_diagonals: C,
        }
    }
    fn histogram(&self) -> HashMap<Point, usize> {
        let mut m = HashMap::new();

        self.lines
            .iter()
            .flat_map(|line| line.iter_with_options(self.considering_diagonals))
            .for_each(|p| {
                let ent = m.entry(p).or_default();
                *ent += 1;
            });
        m
    }

    fn dengerous_points(&self) -> Box<dyn Iterator<Item = Point>> {
        let hist = self.histogram();

        Box::new(
            hist.into_iter()
                .filter(|(_, cnt)| *cnt >= 2)
                .map(|(p, _)| p),
        )
    }
}

impl<const C: bool> FromStr for Field<C> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|line| line.parse::<Line>())
            .collect::<Result<Vec<_>, String>>()?;

        Ok(Field {
            lines,
            considering_diagonals: C,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_from_str() {
        let src = "1,2";
        assert_eq!(Point { x: 1, y: 2 }, src.parse().unwrap());
    }

    #[test]
    fn line_from_str() {
        let src = "1,2 -> 3,4";
        assert_eq!(
            Line {
                start: Point { x: 1, y: 2 },
                end: Point { x: 3, y: 4 },
            },
            src.parse().unwrap()
        );
    }

    #[test]
    fn line_iter() {
        assert_eq!(
            Line::new(1, 0, 1, 2).iter().collect::<Vec<_>>(),
            vec![
                Point { x: 1, y: 0 },
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
            ],
        );

        assert_eq!(
            Line::new(0, 1, 2, 1).iter().collect::<Vec<_>>(),
            vec![
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
            ],
        );

        assert_eq!(Line::new(1, 1, 1, 1).iter().collect::<Vec<_>>(), vec![],);

        assert_eq!(Line::new(1, 1, 2, 2).iter().collect::<Vec<_>>(), vec![],);

        assert_eq!(
            Line::new(1, 1, 2, 3)
                .iter_with_options(true)
                .collect::<Vec<_>>(),
            vec![],
        );

        assert_eq!(
            Line::new(1, 1, 2, 2)
                .iter_with_options(true)
                .collect::<Vec<_>>(),
            vec![Point { x: 1, y: 1 }, Point { x: 2, y: 2 },],
        );

        assert_eq!(
            Line::new(3, 1, 1, 3)
                .iter_with_options(true)
                .collect::<Vec<_>>(),
            vec![
                Point { x: 3, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 1, y: 3 }
            ],
        );
    }

    #[test]
    fn field_from_str() {
        let src = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let field = src.parse::<BasicField>().unwrap();

        assert_eq!(
            field,
            BasicField::new(vec![
                Line::new(0, 9, 5, 9),
                Line::new(8, 0, 0, 8),
                Line::new(9, 4, 3, 4),
                Line::new(2, 2, 2, 1),
                Line::new(7, 0, 7, 4),
                Line::new(6, 4, 2, 0),
                Line::new(0, 9, 2, 9),
                Line::new(3, 4, 1, 4),
                Line::new(0, 0, 8, 8),
                Line::new(5, 5, 8, 2),
            ],),
        );
    }

    #[test]
    fn example_case() {
        let src = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let field = src.parse::<BasicField>().unwrap();

        assert_eq!(5, field.dengerous_points().count());
    }

    #[test]
    fn example_case_part2() {
        let src = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let field = src.parse::<DiagonalField>().unwrap();

        assert_eq!(12, field.dengerous_points().count());
    }
}
