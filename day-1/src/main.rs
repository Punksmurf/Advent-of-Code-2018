use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    let input = read_input().unwrap();
    let silver = solve_silver(&input);
    let gold = solve_gold_naive(&input);
    println!("Silver: {}", silver);
    println!("Gold: {}", gold);
}

fn read_input() -> Result<Vec<i32>, io::Error> {
    let file = File::open("input.txt")?;
    let contents = BufReader::new(&file);

    return Ok(contents.lines().map(|l| {
        return FromStr::from_str(l.unwrap().as_str()).unwrap();
    }).collect());
}

fn solve_silver(numbers: &Vec<i32>) -> i32 {
    return numbers.iter().sum();
}

fn solve_gold_naive(numbers: &Vec<i32>) -> i32 {
    let mut seen = HashSet::new();

    let mut i = 0;

    let mut cur = 0;

    let mut loop_count = 0;

    while !seen.contains(&cur) {
        seen.insert(cur);
//        println!("{}", cur);

        cur += numbers[i];

        i += 1;
        if i == numbers.len() {
            loop_count += 1;
            if loop_count %10 == 0 {
                println!("Loop count: {}", loop_count);
            }
        }
        i %= numbers.len();
    }
//    println!("Done: {}", cur);
    println!("Loops: {}", loop_count);

    return cur;
}

#[test]
fn test_silver() {
    assert_eq!(3, solve_silver(&vec![1, -2, 3, 1]));
    assert_eq!(3, solve_silver(&vec![1, 1, 1]));
    assert_eq!(0, solve_silver(&vec![1, 1, -2]));
    assert_eq!(-6, solve_silver(&vec![-1, -2, -3]));
}

#[test]
fn test_gold() {
    assert_eq!(2, solve_gold_naive(&vec![1, -2, 3, 1]));
    assert_eq!(0, solve_gold_naive(&vec![1, -1]));
    assert_eq!(10, solve_gold_naive(&vec![3, 3, 4, -2, -4]));
    assert_eq!(5, solve_gold_naive(&vec![-6, 3, 8, 5, -6]));
    assert_eq!(14, solve_gold_naive(&vec![7, 7, -2, -7, -4]));
}