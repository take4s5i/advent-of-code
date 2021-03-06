mod y2021;

use std::{collections::HashMap, env};

fn main() {
    let m = get_map();
    if let Some(p) = env::args().nth(1) {
        let solve = m
            .get(&p)
            .expect(&format!("problem '{}' is not found", &p).to_owned());
        solve();
    } else {
        println!("Available problems:");
        for k in m.keys() {
            println!("{}", k);
        }
    }
}

macro_rules! make_map {
    ($($name:expr => $f:expr),+) => {
        {
            use std::collections::HashMap;
            let mut m:HashMap<String, fn()>  = HashMap::new();
            $(m.insert($name.to_string(), $f as fn());)+
            m
        }
    };
}

fn get_map() -> HashMap<String, fn()> {
    return make_map! (
        "y2021/day01" => y2021::day01::solve,
        "y2021/day01_2" => y2021::day01_2::solve,
        "y2021/day02" => y2021::day02::solve,
        "y2021/day02_2" => y2021::day02_2::solve,
        "y2021/day03" => y2021::day03::solve,
        "y2021/day03_2" => y2021::day03_2::solve,
        "y2021/day04" => y2021::day04::solve,
        "y2021/day04_2" => y2021::day04::solve_part2,
        "y2021/day05" => y2021::day05::solve,
        "y2021/day05_2" => y2021::day05::solve_part2,
        "y2021/day06" => y2021::day06::solve,
        "y2021/day06_2" => y2021::day06::solve_part2,
        "y2021/day07" => y2021::day07::solve,
        "y2021/day07_2" => y2021::day07::solve_part2
    );
}
