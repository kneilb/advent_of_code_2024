// https://adventofcode.com/2024/day/1

fn day1(input: &str) -> i32 {
    let mut left_vec: Vec<i32> = input.lines().map(|line| line.split_whitespace().nth(0).unwrap().parse().unwrap()).collect();
    let mut right_vec: Vec<i32> = input.lines().map(|line| line.split_whitespace().nth(1).unwrap().parse().unwrap()).collect();
    left_vec.sort();
    right_vec.sort();

    return left_vec.into_iter().zip(right_vec.into_iter()).map(|(l, r)| { (r - l).abs() }).sum();
}

fn main() {
    let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
    let result = day1(&data);
    assert_eq!(result, 11);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_example() {
	let data = std::fs::read_to_string("data/day1_example.txt").unwrap();
	let result = day1(&data);
	assert_eq!(result, 11);
    }

    #[test]
    fn day1_real() {
	let data = std::fs::read_to_string("data/day1.txt").unwrap();
	let result = day1(&data);
	assert_eq!(result, 2196996);
    }
}
