extern crate itertools;
use itertools::Itertools;

fn main()
{
    println!("{}",std::fs::read_to_string(&String::from("day4/input/input.txt")).unwrap().lines().map(|line|line.split(",").map(|a|a.split("-").map(|b|b.parse::<u32>().unwrap()).collect_tuple().unwrap()).collect_tuple().unwrap()).map(|((a,b),(c,d))|((a>=c&&a<=d)||(c>=a&&c<=b))as(u32)).sum::<u32>());
}