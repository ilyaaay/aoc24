use std::{collections::HashMap, fs, io, iter::zip};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", day1(&input));
    println!("{}", day2(&input));

    Ok(())
}

fn day1(input: &str) -> u32 {
    let (first_column, second_column): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|x| (x[0], x[1]))
        .unzip();

    let mut first_column = first_column;
    let mut second_column = second_column;

    first_column.sort();
    second_column.sort();

    zip(first_column, second_column)
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

fn day2(input: &str) -> i32 {
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

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

    #[test]
    fn day1_test() {
        assert_eq!(day1(INPUT), 11);
    }

    #[test]
    fn day2_test() {
        assert_eq!(day2(INPUT), 31);
    }
}
