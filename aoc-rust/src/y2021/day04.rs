use std::io::Read;
use std::str::FromStr;

pub fn solve() {
    let mut src = String::new();
    std::io::stdin().lock().read_to_string(&mut src).unwrap();

    let mut game = src.parse::<BingoGame>().unwrap();
    let final_score = game.play();
    println!("{}", final_score);
}

pub fn solve_part2() {
    let mut src = String::new();
    std::io::stdin().lock().read_to_string(&mut src).unwrap();

    let mut game = src.parse::<BingoGame>().unwrap();
    let final_score = game.play_part2();
    println!("{}", final_score);
}

#[derive(PartialEq, Eq, Debug)]
struct NumberGenerator {
    nums: Vec<u8>,
}

impl FromStr for NumberGenerator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(",")
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().parse::<u8>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("NumberGenerator parse error: {}", e).to_string())?;
        Ok(NumberGenerator { nums })
    }
}

const SIZE: usize = 5;

#[derive(PartialEq, Eq, Debug)]
struct Pos(usize);

impl Pos {
    fn row(&self) -> usize {
        self.0 / SIZE
    }

    fn col(&self) -> usize {
        self.0 % SIZE
    }
}

#[derive(PartialEq, Eq, Debug, Default)]
struct Board {
    board: [u8; SIZE * SIZE],
    rows: [u8; SIZE],
    cols: [u8; SIZE],
}

impl Board {
    fn open(&mut self, num: u8) {
        if let Some(pos) = self.find_pos(num) {
            let row = pos.row();
            let col = pos.col();

            self.rows[row] = self.rows[row] | (1 << col);
            self.cols[col] = self.cols[col] | (1 << row);
        }
    }

    fn is_winning(&self) -> bool {
        for n in self.rows.into_iter() {
            if n == (1 << SIZE) - 1 {
                return true;
            }
        }

        for n in self.cols.into_iter() {
            if n == (1 << SIZE) - 1 {
                return true;
            }
        }

        false
    }

    fn is_marked_at(&self, pos: Pos) -> bool {
        let row = pos.row();
        let col = pos.col();

        if (self.rows[row] & (1 << col)) == 0 {
            return false;
        }

        if (self.cols[col] & (1 << row)) == 0 {
            return false;
        }

        true
    }

    fn all_unmarked<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        (0..(SIZE * SIZE)).filter_map(|x| {
            let pos = Pos(x);
            if self.is_marked_at(pos) {
                None
            } else {
                Some(self.board[x])
            }
        })
    }

    fn find_pos(&self, num: u8) -> Option<Pos> {
        for n in 0..self.board.len() {
            if self.board[n] == num {
                return Some(Pos(n));
            }
        }
        None
    }
}

impl FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::default();
        let lines: Vec<_> = s.lines().filter(|s| !s.is_empty()).collect();

        if lines.len() != SIZE {
            let msg = format!(
                "Board parse error: Board size must be {0}x{0}. but the board has hight of {1}",
                SIZE,
                lines.len()
            );
            return Err(msg.to_string());
        }

        for (row, line) in lines.iter().enumerate() {
            let cols: Vec<_> = line.split(' ').filter(|s| !s.is_empty()).collect();

            if cols.len() != SIZE {
                let msg = format!(
                    "Board parse error: Board size must be {0}x{0}. but the board has width of {1} on row of {2}",
                    SIZE,
                    cols.len(),
                    row,
                );
                return Err(msg.to_string());
            }

            for (col, d) in cols.iter().enumerate() {
                board.board[row * SIZE + col] = d
                    .parse::<u8>()
                    .map_err(|e| format!("Board parse error: {}", e))?;
            }
        }

        Ok(board)
    }
}

struct BingoGame {
    gen: NumberGenerator,
    boards: Vec<Board>,
}

impl BingoGame {
    fn play(&mut self) -> u64 {
        for n in self.gen.nums.iter().cloned() {
            for b in self.boards.iter_mut() {
                b.open(n);

                if b.is_winning() {
                    return b.all_unmarked().map(|x| x as u64).sum::<u64>() * (n as u64);
                }
            }
        }

        panic!("all of boards had never won.");
    }

    fn play_part2(&mut self) -> u64 {
        let mut playings: Vec<_> = (0..self.boards.len()).collect();
        let mut winners = vec![];
        let mut remains = vec![];

        for n in self.gen.nums.iter().cloned() {
            winners.clear();
            remains.clear();

            for i in playings.iter().cloned() {
                let b = &mut self.boards[i];
                b.open(n);

                if b.is_winning() {
                    winners.push(i);
                } else {
                    remains.push(i);
                }
            }

            if remains.len() == 0 && winners.len() == 1 {
                return self.boards[winners[0]]
                    .all_unmarked()
                    .map(|x| x as u64)
                    .sum::<u64>()
                    * (n as u64);
            }

            playings = remains.iter().cloned().collect();
        }

        panic!("all of boards had never won.");
    }
}

impl FromStr for BingoGame {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boards = vec![];
        let mut lines = s.lines();
        let mut buf = String::new();

        let gen =
            NumberGenerator::from_str(lines.next().ok_or("The first line must be numbers.")?)?;

        for line in lines {
            if line.is_empty() {
                if !buf.is_empty() {
                    boards.push(buf.parse::<Board>()?);
                }
                buf.clear();
            } else {
                buf.push_str(line);
                buf.push('\n');
            }
        }

        if !buf.is_empty() {
            boards.push(buf.parse::<Board>()?);
        }

        Ok(BingoGame { gen, boards })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_generator_from_str() {
        let s = "";
        assert_eq!(
            NumberGenerator::from_str(&s),
            Ok(NumberGenerator { nums: vec![] })
        );

        let s = " 1,2,3,4,5, 0 ,11 ";
        assert_eq!(
            NumberGenerator::from_str(&s),
            Ok(NumberGenerator {
                nums: vec![1, 2, 3, 4, 5, 0, 11],
            })
        );

        let s = "a";
        assert_eq!(NumberGenerator::from_str(&s).is_err(), true);
    }

    #[test]
    fn board_from_str() {
        let s = r#"
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;

        let mut expect = Board::default();
        expect.board = [
            22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15,
            19,
        ];
        assert_eq!(Board::from_str(&s), Ok(expect),);
    }

    #[test]
    fn board_find_pos() {
        let s = r#"
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;
        let board = Board::from_str(&s).unwrap();
        assert_eq!(board.find_pos(22), Some(Pos(0)));
        assert_eq!(board.find_pos(8), Some(Pos(5)));
        assert_eq!(board.find_pos(19), Some(Pos(24)));
        assert_eq!(board.find_pos(99), None);
    }

    #[test]
    fn board_open() {
        let s = r#"
22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19
"#;
        let mut board = Board::from_str(&s).unwrap();
        board.open(22);
        assert_eq!(
            (1, 1, false),
            (board.rows[0], board.cols[0], board.is_winning()),
        );

        board.open(8);
        assert_eq!(
            (1, 3, false),
            (board.rows[0], board.cols[0], board.is_winning()),
        );

        board.open(17);
        assert_eq!(
            (5, 3, false),
            (board.rows[0], board.cols[0], board.is_winning()),
        );

        board.open(23);
        board.open(14);
        board.open(3);
        board.open(20);
        assert_eq!(board.cols[2], 31);
        assert_eq!(board.is_winning(), true);
    }

    #[test]
    fn example_case() {
        let src = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let mut game = src.parse::<BingoGame>().unwrap();
        assert_eq!(game.boards.len(), 3);

        let final_score = game.play();
        assert_eq!(final_score, 4512);
    }

    #[test]
    fn example_case_part2() {
        let src = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

        let mut game = src.parse::<BingoGame>().unwrap();
        assert_eq!(game.boards.len(), 3);

        let final_score = game.play_part2();
        assert_eq!(final_score, 1924);
    }
}
