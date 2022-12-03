extern crate regex;
use regex::Regex;

fn main()
{
    println!("{:?}", std::fs::read_to_string(&String::from("day3/input/input.txt")).expect("Error reading file.").lines().map(| line | (&line[0..(line.len() / 2)], &line[(line.len() / 2)..line.len()]) ).map(| (a, b) | b.as_bytes()[Regex::new(&format!("{}{}{}","[",a,"]")).unwrap().shortest_match(b).expect("No match found!") - 1]).map(| value | if value >= 'a' as u8 { (value - ('a' as u8) + 1) as u32 } else { (value - ('A' as u8) + 27) as u32 }).sum::<u32>());
}