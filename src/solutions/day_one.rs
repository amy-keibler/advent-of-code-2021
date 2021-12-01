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
    println!("{} depth increases", increases);

    let window_increases = report_window_depth_increases(&depths);
    println!("{} window depth increases", window_increases);
}

fn report_depth_increases(depths: &Vec<u32>) -> u32 {
    depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(current_depth, next_depth)| current_depth < next_depth)
        .count() as u32
}

fn report_window_depth_increases(depths: &Vec<u32>) -> u32 {
    let window_depths = depths
        .iter()
        .zip(depths.iter().skip(1).zip(depths.iter().skip(2)))
        .map(|(a, (b, c))| a + b + c)
        .collect();
    report_depth_increases(&window_depths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_example_part_one() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let output = report_depth_increases(&input);

        assert_eq!(output, 7);
    }

    #[test]
    fn test_provided_example_part_two() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let output = report_window_depth_increases(&input);
        assert_eq!(output, 5);
    }
}
