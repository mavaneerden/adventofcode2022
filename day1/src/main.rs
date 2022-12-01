fn main()
{
    let mut max: [u64; 3] = [0; 3];
    let mut local_sum: u64 = 0;
    let contents = std::fs::read_to_string(std::path::Path::new(&String::from("day1/input/input.txt")))
        .expect("Error reading file.");

    for line in contents.lines()
    {
        match line.parse::<u64>()
        {
            Ok(value) => local_sum += value,
            Err(_) =>
            {
                if local_sum > max[1]
                {
                    if local_sum > max[0]
                    {
                        max[2] = max[1];
                        max[1] = max[0];
                        max[0] = local_sum
                    }
                    else
                    {
                        max[2] = max[1];
                        max[1] = local_sum
                    }
                }
                else if local_sum > max[2]
                {
                    max[2] = local_sum
                }

                local_sum = 0
            }
        }
    }

    let sum_total = max[0] + max[1] + max[2];

    println!("{sum_total}");
}