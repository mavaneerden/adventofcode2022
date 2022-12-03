extern crate regex;
extern crate itertools;
use regex::Regex;
use itertools::Itertools;

fn main()
{
    println!("{}", std::fs::read_to_string(&String::from("day3/input/input.txt")).expect("Error reading file.").lines().batching(| it | match it.next() { None => None, Some(a) => Some((a, it.next().expect(""), it.next().expect(""))) }).map(| (a, b, c) | (Regex::new(&format!("{}{}{}","[",a,"]{1}")).unwrap().find_iter(b).map(| value | String::from(value.as_str()) ).reduce(| x, y | format!("{}{}", x, y)).expect(""), c)).map(| (a, b) | b.as_bytes()[Regex::new(&format!("{}{}{}","[",a,"]")).unwrap().shortest_match(b).expect("") - 1]).map(| value | if value >= 'a' as u8 { (value - ('a' as u8) + 1) as u32 } else { (value - ('A' as u8) + 27) as u32 }).sum::<u32>());
}