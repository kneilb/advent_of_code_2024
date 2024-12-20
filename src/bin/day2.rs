// https://adventofcode.com/2024/day/2

#[derive(Debug)]
struct Report {
    data: Vec<i32>,
}

impl Report {
    fn parse(line: &str) -> Report {
	let data: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

	Report {
	    data,
	}
    }

    fn is_safe(&self) -> bool {
	let mut was_increasing: Option<bool> = None;
	let mut last: Option<i32> = None;

	for d in &self.data {

	    print!("d: {}\n", d);
	    if last != None {
		let diff: i32 = last.unwrap() - d;
		let increasing: bool = diff < 0;

		print!("last: {:?}, diff: {}, increasing: {}, was_increasing: {:?}\n", last, diff, increasing, was_increasing);

		if diff == 0 || diff.abs() > 3 {
		    print!("diff outside bounds: {}\n", diff);
		    return false;
		}

		if was_increasing != None && increasing != was_increasing.unwrap() {
		    print!("direction change!\n");
		    return false;
		}

		was_increasing = Some(increasing);
	    }
	    last = Some(*d);
	}

	return true;
    }
}

fn day2(input: &str) -> usize {
    input
	.lines()
	.map(|line| Report::parse(line))
	.filter(|report| report.is_safe())
	.collect::<Vec<_>>()
	.len()
}

fn main() {
    let data = std::fs::read_to_string("data/day2_example.txt").unwrap();
    let result = day2(&data);
    assert_eq!(result, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2_example() {
	let data = std::fs::read_to_string("data/day2_example.txt").unwrap();
	let result = day2(&data);
	assert_eq!(result, 2);
    }

    #[test]
    fn day2_real() {
	let data = std::fs::read_to_string("data/day2.txt").unwrap();
	let result = day2(&data);
	assert_eq!(result, 670);
    }
}
