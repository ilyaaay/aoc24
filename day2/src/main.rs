use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("./src/input.txt")?;

    println!("{}", first_part(&input));
    println!("{}", second_part(&input));

    Ok(())
}

enum ReportState {
    Safe,
    Unsafe,
}

fn first_part(input: &str) -> i32 {
    let nums = input.lines().map(|x| {
        x.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });
    // .map(|nums| );

    // println!("{:?}", nums);

    1
}

fn check_nums_seq(nums: Vec<i32>) {
    let mut iter = nums.iter();

    for i in iter.clone() {
        let x = iter.next();

        println!("{:?}", x);

        break;
    }
}

fn second_part(input: &str) -> i32 {
    1
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
