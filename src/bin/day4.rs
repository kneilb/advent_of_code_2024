// https://adventofcode.com/2024/day/4

// Search for XMAS
// 1. Find X
// 2. Look in all 8 directions for M
// 3. Look in same direction for A
// 4. Look in same direction for S
// A letter can be used more than once!!
// So don't stop checking other directions once you've found an M at step 2!

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),   // Up
    (1, 1),   // Up / right
    (0, 1),   // Right
    (-1, 1),  // Down / right
    (-1, 0),  // Down
    (-1, -1), // Down / left
    (0, -1),  // Left
    (1, -1),  // Up / left
];

fn check_for_letter(letter: char, x: i32, y: i32, grid: &Vec<Vec<char>>, width: usize, height: usize) -> bool {
    if x < 0 || y < 0 {
	return false;
    }

    let x = x as usize;
    let y = y as usize;

    if x >= width || y >= height  {
	return false;
    }

    grid[y][x] == letter
}

fn day4(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Assumes all row & columns have the same length...
    let height: usize = grid.len();
    let width: usize = grid[0].len();

    print!("{:?}\n", grid);
    let mut matches: usize = 0;

    for y in 0..height {
	for x in 0..width {
	    for d in DIRECTIONS {
		let mut search_x: i32 = x as i32;
		let mut search_y: i32 = y as i32;

		for l in ['X', 'M', 'A', 'S'] {
		    if l != 'X' {
			search_x += d.1;
			search_y += d.0;
		    }

		    if check_for_letter(l, search_x, search_y, &grid, width, height) {
			print!("FOUND {} at ({}, {})\n", l, search_x, search_y);
			if l == 'S' {
			    matches += 1;
			}
		    }
		    else {
			break;
		    }
		}
	    }
	}
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4_example() {
	let data = std::fs::read_to_string("data/day4_example.txt").unwrap();
	let result = day4(&data);
	assert_eq!(result, 18);
    }

    #[test]
    fn day4_real() {
	let data = std::fs::read_to_string("data/day4.txt").unwrap();
	let result = day4(&data);
	assert_eq!(result, 2521);
    }
}
