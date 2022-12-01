extern crate itertools;
use itertools::Itertools;

fn main()
{
    println!("{}", std::fs::read_to_string(std::path::Path::new(&String::from("day1/input/input.txt"))).expect("Error reading file.").lines().map(| val | { val.parse::<i64>().unwrap_or(-1) }).coalesce(| a, b | if b >= 0 { Ok(a + b) } else { Err((a, b)) } ).sorted_by(| a, b | b.cmp(a)).take(3).sum::<i64>());
}