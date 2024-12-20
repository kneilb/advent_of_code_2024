// https://adventofcode.com/2024/day/3

use regex::Regex;

fn day3(input: &str) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    re.captures_iter(input).map(|c| c.extract()).map(|(_, [a, b])| a.parse::<usize>().unwrap_or(0) * b.parse::<usize>().unwrap_or(0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3_example() {
	let data = std::fs::read_to_string("data/day3_example.txt").unwrap();
	let result = day3(&data);
	assert_eq!(result, 161);
    }

    #[test]
    fn day3_real() {
	let data = std::fs::read_to_string("data/day3.txt").unwrap();
	let result = day3(&data);
	assert_eq!(result, 174336360);
    }
}
