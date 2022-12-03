fn main()
{
    println!("{}",std::fs::read_to_string(std::path::Path::new(&String::from("day2/input/input.txt"))).expect("Error reading file.").lines().map(|line|{(line.split(" ").take(2).nth(0).expect("Error:no 1st element!"),line.split(" ").take(2).nth(1).expect("Error: no 1st element!"))}).map(|chars|{return match chars.1{"X"=>if chars.0=="A"{3+0}else if chars.0=="B"{1+0}else{2+0},"Y"=>if chars.0=="A"{1+3}else if chars.0=="B"{2+3}else{3+3},"Z"=>if chars.0=="A"{2+6}else if chars.0=="B"{3+6}else{1+6},_=>panic!("Not possible ;)")}}).sum::<i32>());
}