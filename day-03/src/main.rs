use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = parse_input()?;

    // for c in &password_infos {
    //     for x in c {
    //         print!("{}", x);
    //     }
    //     println!("")
    // }

    part_one(&input);

    Ok(())
}

fn part_one(input: &Vec<Vec<char>>) {
    let mut x = 0;
    let mut y = 0;
    let mut acc = 0;

    loop {
        x += 1;
        y += 3;
        if x >= input.len() {
            break;
        }
        if input[x][y % input[0].len()] == '#' {
            acc += 1;
        }
    }
    println!("{}", acc);
}

fn parse_input() -> Result<Vec<Vec<char>>, Error> {
    let input = fs::read_to_string("input.txt")?;
    let numbers = input.lines().map(|line| line.chars().collect()).collect();
    Ok(numbers)
}
