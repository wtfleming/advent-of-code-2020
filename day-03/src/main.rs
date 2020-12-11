use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = parse_input()?;

    part_one(&input);
    part_two(&input);

    Ok(())
}

fn part_one(input: &Vec<Vec<char>>) {
    let result = num_trees_encountered(input, 3, 1);
    println!("{}", result);
}

fn part_two(input: &Vec<Vec<char>>) {
    let values: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result: usize = values
        .iter()
        .map(|(x, y)| num_trees_encountered(input, *x as usize, *y as usize))
        .product();
    println!("{}", result);
}

fn num_trees_encountered(input: &Vec<Vec<char>>, y_step: usize, x_step: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut acc = 0;

    loop {
        x += x_step;
        y += y_step;
        if x >= input.len() {
            break;
        }
        if input[x][y % input[0].len()] == '#' {
            acc += 1;
        }
    }
    acc
}

fn parse_input() -> Result<Vec<Vec<char>>, Error> {
    let input = fs::read_to_string("input.txt")?;
    let numbers = input.lines().map(|line| line.chars().collect()).collect();
    Ok(numbers)
}
