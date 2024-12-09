use std::{fs, ops::ControlFlow};

const DEBUG: bool = false;

fn main() {
    let contents = fs::read_to_string("inputs/02.txt")
        .expect("should be able to read the file");

    solve1(contents.clone());
    solve2(contents.clone());
}

fn solve1(contents: String) {
    let answer = contents.lines()
        .map(|report| {
            let valid = is_valid_report(report.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()));
            if DEBUG {println!("Row: {report} is {valid}");}
            return valid;
        }).filter(|x| *x)
        .count();

    println!("Answer to question 1: {answer}")
}

fn solve2(contents: String) {
    let answer = contents.lines()
        .map(|report| {
            let split_report = report.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap());
            let str_report = split_report.clone().into_iter().fold(String::new(), |a, b| a + &b.to_string() + " ");

            if is_valid_report(split_report.clone()) {
                if DEBUG {println!("Row: {str_report} is true");}
                return true;
            }

            for i in 0..split_report.clone().count() {
                let mut current_split_report: Vec<i32> = split_report.clone().into_iter().collect();
                current_split_report.remove(i);
                let has_valid = is_valid_report(current_split_report.clone().into_iter());
                if has_valid {
                    let str_report = current_split_report.into_iter().fold(String::new(), |a, b| a + &b.to_string() + " ");
                    if DEBUG {println!("Row: {str_report} is true");}
                    return true
                }
            }
            

            if DEBUG {println!("Row: {str_report} is false");}
            return false;

        })
        .filter(|x| *x)
        .count();

    println!("Answer to question 2: {answer}");
}

fn is_valid_report(report: impl Iterator<Item = i32> + Clone) -> bool {
    let mut safe = true;
    let mut p_direction = 0;

    report.clone().enumerate().try_for_each(|(i, level)| {
        let next_level= report.clone().nth(i + 1);
        if next_level.is_none() {
            return ControlFlow::Break(())
        }

        let distance = next_level.unwrap() - level;
        let direction = if distance.is_positive() { 1 } else { -1 };
        
        if p_direction == 0 {
            p_direction = direction;
        }

        if direction != p_direction || distance.abs() < 1 || distance.abs() > 3 {
            safe = false;
            return ControlFlow::Break(());
        }

        p_direction = direction;
        return ControlFlow::Continue(());
    });


    return safe;
}