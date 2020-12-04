use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};

struct PasswordInfo {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

fn main() -> Result<(), Error> {
    let password_infos = input_to_password_info()?;
    part_one(&password_infos);

    Ok(())
}

fn part_one(numbers: &[PasswordInfo]) {
    let results: Vec<&PasswordInfo> = numbers.iter().filter(|x| is_password_valid(x)).collect();
    println!("{}", results.len());
}

fn input_to_password_info() -> Result<Vec<PasswordInfo>, Error> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let numbers = buffered
        .lines()
        .map(|line| {
            let re = Regex::new(r"(\d+)-(\d+) (.): (.+)").unwrap();

            let x = line.unwrap();
            let cap = re.captures(&x).unwrap();

            PasswordInfo {
                min: cap[1].parse::<usize>().unwrap(),
                max: cap[2].parse::<usize>().unwrap(),
                letter: cap[3].chars().next().unwrap(),
                password: cap[4].to_string(),
            }
        })
        .collect();
    Ok(numbers)
}

fn is_password_valid(pw: &PasswordInfo) -> bool {
    let mut pass = pw.password.clone();
    pass.retain(|c| c == pw.letter);

    // println!(
    //     "{} {} {} {} {}",
    //     pw.letter, pw.min, pw.max, pw.password, pass
    // );

    pass.len() >= pw.min && pass.len() <= pw.max
}