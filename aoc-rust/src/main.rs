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
        "y2021/day1" => y2021::day1::solve,
        "y2021/day1_2" => y2021::day1_2::solve
    );
}
