use std::io;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

fn main() {
    let input = read_input().unwrap();
    let silver = solve_silver(&input);
    let gold = solve_gold(&input);
    println!("Silver: {}", silver);
    println!("Gold: {}", gold);
}

fn read_input() -> Result<Vec<String>, io::Error> {
    let file = File::open("input.txt")?;
    let contents = BufReader::new(&file);

    return Ok(contents.lines()
        .map(|l| l.unwrap())
        .collect());
}

fn solve_silver(ids: &Vec<String>) -> i32 {
    let twos = ids_with_char_count(ids, 2);
    let threes = ids_with_char_count(ids, 3);
    return twos * threes;

}

fn ids_with_char_count(ids: &Vec<String>, count: i32) -> i32 {
    return ids.iter()
        .filter(|l| contains_char_count(&l, count))
        .count() as i32;
}

fn contains_char_count(id: &String, count: i32) -> bool {
    let counts = get_character_counts(id);
//    dump_character_counts(&counts);
    return counts.values().any(|v| *v == count);
}

fn get_character_counts(id: &String) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    id.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });

    return counts;
}

fn dump_character_counts(counts: &HashMap<char, i32>) {
    counts.iter().for_each( | (k,v) | {
        println!("{} {}", k, v);
    })
}

fn solve_gold(ids: &Vec<String>) -> String {

    for i in 0..ids.len() - 1 {
        for j in i..ids.len() {
            if num_diff_chars(&ids[i], &ids[j]) == 1 {
                return all_equal_characters(&ids[i], &ids[j]);
            }
        }
    }

    panic!("No solution found");
}

fn num_diff_chars(lhs: &String, rhs: &String) -> i32 {
    if lhs.len() != rhs.len() {
        panic!("Strings must be of equal length");
    }

    let mut lcs = lhs.chars();
    let mut rcs = rhs.chars();

    let mut count = 0;

    for _ in 0..lhs.len() {
        if lcs.next().unwrap() != rcs.next().unwrap() {
            count = count + 1;
        }
    }

    return count;
}

fn all_equal_characters(lhs: &String, rhs: &String) -> String {
    if lhs.len() != rhs.len() {
        panic!("Strings must be of equal length");
    }

    let mut lcs = lhs.chars();
    let mut rcs = rhs.chars();

    let mut result = String::new();

    for _ in 0..lhs.len() {
        let c = lcs.next().unwrap();
        if c == rcs.next().unwrap() {
            result.push(c);
        }
    }

    return result;

}

#[test]
fn test_contains_2() {
    let c = 2;
    assert_eq!(false, contains_char_count(&"abcdef".to_string(), c));
    assert_eq!(true, contains_char_count(&"bababc".to_string(), c));
    assert_eq!(true, contains_char_count(&"abbcde".to_string(), c));
    assert_eq!(false, contains_char_count(&"abcccd".to_string(), c));
    assert_eq!(true, contains_char_count(&"aabcdd".to_string(), c));
    assert_eq!(true, contains_char_count(&"abcdee".to_string(), c));
    assert_eq!(false, contains_char_count(&"ababab".to_string(), c));
}

#[test]
fn test_contains_3() {
    let c = 3;
    assert_eq!(false, contains_char_count(&"abcdef".to_string(), c));
    assert_eq!(true, contains_char_count(&"bababc".to_string(), c));
    assert_eq!(false, contains_char_count(&"abbcde".to_string(), c));
    assert_eq!(true, contains_char_count(&"abcccd".to_string(), c));
    assert_eq!(false, contains_char_count(&"aabcdd".to_string(), c));
    assert_eq!(false, contains_char_count(&"abcdee".to_string(), c));
    assert_eq!(true, contains_char_count(&"ababab".to_string(), c));
}

#[test]
fn test_ids_with_char_count() {
    let ids = vec![
        "abcdef".to_string(),
        "bababc".to_string(),
        "abbcde".to_string(),
        "abcccd".to_string(),
        "aabcdd".to_string(),
        "abcdee".to_string(),
        "ababab".to_string(),
    ];
    assert_eq!(4, ids_with_char_count(&ids, 2));
    assert_eq!(3, ids_with_char_count(&ids, 3));
}

#[test]
fn test_silver() {
    let ids = vec![
        "abcdef".to_string(),
         "bababc".to_string(),
         "abbcde".to_string(),
         "abcccd".to_string(),
         "aabcdd".to_string(),
         "abcdee".to_string(),
         "ababab".to_string(),
    ];
    assert_eq!(12, solve_silver(&ids));
}

#[test]
fn test_num_diff_chars() {
    assert_eq!(0, num_diff_chars(&"abc".to_string(), &"abc".to_string()));
    assert_eq!(1, num_diff_chars(&"abc".to_string(), &"abd".to_string()));
    assert_eq!(2, num_diff_chars(&"abcde".to_string(), &"axcye".to_string()));
    assert_eq!(1, num_diff_chars(&"fghij".to_string(), &"fguij".to_string()));
}

#[test]
fn test_all_equal_characters() {
    assert_eq!("abc", all_equal_characters(&"abc".to_string(), &"abc".to_string()));
    assert_eq!("ace", all_equal_characters(&"abcde".to_string(), &"axcye".to_string()));
    assert_eq!("fgij", all_equal_characters(&"fghij".to_string(), &"fguij".to_string()));
}

#[test]
fn test_gold() {
    let ids = vec![
        "abcde".to_string(),
        "fghij".to_string(),
        "klmno".to_string(),
        "pqrst".to_string(),
        "fguij".to_string(),
        "axcye".to_string(),
        "wvxyz".to_string(),
    ];
    assert_eq!("fgij", solve_gold(&ids));
}