use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let numbers = input_to_vec()?;
    part_one(&numbers);
    part_two(&numbers);

    Ok(())
}

fn part_one(numbers_in: &[i32]) {
    let mut numbers = numbers_in.to_vec();
    while !numbers.is_empty() {
        let num = numbers.pop().unwrap();
        let needed = 2020 - num;
        if numbers.contains(&needed) {
            println!("{}", num * needed);
            return;
        }
    }
}

fn part_two(numbers: &[i32]) {
    for (pos, e) in numbers.iter().enumerate() {
        let slice_one = &numbers[pos..(numbers.len() - 1)];
        for (pos2, e2) in slice_one.iter().enumerate() {
            let slice_two = &slice_one[pos2..(slice_one.len() - 1)];
            for (_pos3, e3) in slice_two.iter().enumerate() {
                if (e + e2 + e3) == 2020 {
                    println!("{}", e * e2 * e3);
                    return;
                }
            }
        }
    }
}

fn input_to_vec() -> Result<Vec<i32>, Error> {
    let input = File::open("input.txt")?;
    let buffered = BufReader::new(input);
    let numbers = buffered
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();
    Ok(numbers)
}
