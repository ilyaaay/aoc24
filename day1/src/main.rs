use std::{collections::HashMap, fs, io, iter::zip};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_part(&input));
    println!("{}", second_part(&input));

    Ok(())
}

fn first_part(input: &str) -> u32 {
    let (mut first_column, mut second_column): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|x| (x[0], x[1]))
        .unzip();

    first_column.sort();
    second_column.sort();

    zip(first_column, second_column)
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

fn second_part(input: &str) -> i32 {
    let mut counts = HashMap::new();
    let mut first_column = Vec::new();

    for i in input.lines() {
        let nums = i
            .split_whitespace()
            .filter_map(|x| x.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        first_column.push(nums[0]);
        *counts.entry(nums[1]).or_insert(0) += 1;
    }

    let mut result = 0;

    for i in first_column.iter() {
        if let Some(x) = counts.get(i) {
            result += i * x;
        }
    }

    result
}

fn parse_input_first_part(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|x| x.split_once("   ").unwrap())
        .collect()
}

fn first_part_experimental(input: &str) -> u32 {
    let (mut first, mut second): (Vec<i32>, Vec<i32>) = parse_input_first_part(input)
        .into_iter()
        .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
        .unzip();

    first.sort();
    second.sort();

    zip(first, second).map(|(x, y)| x.abs_diff(y)).sum()
}

const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

#[test]
fn day1_test() {
    assert_eq!(first_part(INPUT), 11);
}

#[test]
fn day2_test() {
    assert_eq!(second_part(INPUT), 31);
}

#[test]
fn day1_test_experimental() {
    assert_eq!(first_part_experimental(INPUT), 11);
}
