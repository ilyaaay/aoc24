use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_part(&input));
    println!("{}", second_part(&input));

    Ok(())
}

fn first_part(input: &str) -> i32 {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect::<Vec<i32>>()
        })
        .fold(0, |x, nums| {
            match (nums.is_sorted_by(|x, y| x < y) || nums.is_sorted_by(|x, y| x > y))
                && (nums.is_sorted_by(|x, &y| x.abs_diff(y) >= 1 && x.abs_diff(y) <= 3))
            {
                true => x + 1,
                false => x,
            }
        })
}

fn second_part(input: &str) -> i32 {
    let mut result = 0;

    for i in input.lines() {
        let nums = i
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>();
    }

    result
}

const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[test]
fn test_first_part() {
    assert_eq!(first_part(INPUT), 2);
}

#[test]
fn test_second_part() {
    assert_eq!(second_part(INPUT), 4);
}
