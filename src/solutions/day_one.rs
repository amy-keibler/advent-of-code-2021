use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // first puzzle
    let input = File::open("inputs/day_one_part_one.txt").expect("Failed to open the file");
    let depths = BufReader::new(input)
        .lines()
        .filter_map(Result::ok)
        .map(|l| u32::from_str_radix(&l, 10))
        .filter_map(Result::ok)
        .collect();

    let increases = report_depth_increases(&depths);
    println!("{} increases", increases);
}

fn report_depth_increases(depths: &Vec<u32>) -> u32 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(current_depth, next_depth)| current_depth < next_depth)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_example() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let output = report_depth_increases(&input);

        assert_eq!(output, 7);
    }
}
