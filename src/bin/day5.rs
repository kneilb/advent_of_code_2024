// https://adventofcode.com/2024/day/5

#[derive(Debug)]
struct Ordering {
    before: usize,
    after: usize,
}

impl Ordering {
    fn validate(self: &Ordering, sequence: &Vec<usize>) -> bool {
	let mut found_after = false;
	for s in sequence {
	    if *s == self.after {
		found_after = true;
	    }
	    if found_after && *s == self.before {
		return false;
	    }
	}
	true
    }

    fn parse(line: &str) -> Option<Ordering> {
	let mut tokens = line.split("|");
	if tokens.clone().count() != 2 {
	    None
	}
	else {
	    let before = tokens.next().unwrap().parse().unwrap();
	    let after = tokens.next().unwrap().parse().unwrap();
	    Some(Ordering {
		before,
		after,
	    })
	}
    }
}

fn parse_update(line: &str) -> Option<Vec<usize>> {
    let tokens = line.split(",");
    if tokens.clone().count() < 2 {
	None
    }
    else {
	let update: Vec<usize> = tokens.map(|x| x.parse().unwrap()).collect();

	Some(update)
    }
}

fn day5(input: &str) -> usize {
    let orderings: Vec<Ordering> = input.lines().filter_map(|line| Ordering::parse(line)).collect();
    let updates: Vec<Vec<usize>> = input.lines().filter_map(|line| parse_update(line)).collect();

    let mut total: usize = 0;

    for u in updates {
	if !orderings.iter().any(|o| !o.validate(&u)) {
	    let mid_val = u[u.len() / 2];
	    total += mid_val;

	    print!("Update {:?} is valid! -> {} -> {}\n", u, mid_val, total);
	}
    }

    total
}

fn main() {
    let data = std::fs::read_to_string("data/day5_example.txt").unwrap();
    let result = day5(&data);
    assert_eq!(result, 143);
}

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5_example() {
	let data = std::fs::read_to_string("data/day5_example.txt").unwrap();
	let result = day5(&data);
	assert_eq!(result, 143);
    }

    #[test]
    fn day5_real() {
	let data = std::fs::read_to_string("data/day5.txt").unwrap();
	let result = day5(&data);
	assert_eq!(result, 5588);
    }
}
