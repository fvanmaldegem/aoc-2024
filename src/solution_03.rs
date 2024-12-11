use std::fs;

use regex::{Match, Regex};

const DEBUG: bool = false;
const REGEX01: &str = r"mul\(([0-9]{1,3})\,([0-9]{1,3})\)";
const REGEX02: &str = r"((?<do>do\(\))|(?<dont>don't\(\))|mul\((?<l>[0-9]{1,3})\,(?<r>[0-9]{1,3})\))";

fn main() {
    let contents = fs::read_to_string("inputs/03.txt")
        .expect("should be able to read the file");

    solve_01(contents.clone());
    solve_02(contents);
}

fn solve_01(content: String) {
    let re = Regex::new(REGEX01).unwrap();
    let answer: i32 = re.captures_iter(&content).map(|captures| {
        let (_, [l, r]) = captures.extract();
        if DEBUG {println!("adding {l} * {r} to total.")}
        return l.parse::<i32>().unwrap() * r.parse::<i32>().unwrap();
    }).sum();

    println!("Answer for question 1: {answer}");
}

fn solve_02(content: String) {
    let re = Regex::new(REGEX02).unwrap();
    let mut total = 0;
    let mut active = true;
    for (activate, disable, l, r) in re.captures_iter(&content).map(|captures| {
        let unwrap_parse_i32 = | m: Match<'_> | Some(m.as_str().parse::<i32>().unwrap());
        let l: Option<i32> = captures.name("l").map_or_else(|| None, unwrap_parse_i32);
        let r: Option<i32> = captures.name("r").map_or_else(|| None, unwrap_parse_i32);
        return (
            captures.name("do").is_some(),
            captures.name("dont").is_some(),
            l,
            r
        );
    }) {
        if activate {
            if DEBUG {println!("activating");}
            active = true;
            continue;
        }

        if disable {
            if DEBUG {println!("disabling");}
            active = false;
            continue;
        }

        if active && l.is_some() && r.is_some() {
            if DEBUG {
                
                println!("adding {} * {} to total", l.unwrap(), r.unwrap());
            }
            total += l.unwrap() * r.unwrap();
            continue;
        }
    }

    println!("Answer for question 2: {total}");
}