use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", day1(&input));

    Ok(())
}

fn day1(input: &str) -> i32 {
    let mut result = 0;

    for i in input.lines() {
        let nums = i
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect::<Vec<i32>>();
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::*;

    const INPUT: &str = r#"
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn test_day1() {
        assert_eq!(day1(INPUT), 2);
    }
}
