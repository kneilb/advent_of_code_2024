// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    obstacles: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
	let obstacles: Vec<Vec<bool>> = input.lines().map(|l| l.chars().map(|c| c == '#').collect()).collect();
	let height = obstacles.len();
	let width = obstacles[0].len();
	Map {
	    obstacles,
	    width,
	    height
	}
    }

}

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
enum Facing {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

impl Facing {
    fn next(current: Facing) -> Facing {
	match current {
	    Facing::UP => Facing::RIGHT,
	    Facing::RIGHT => Facing::DOWN,
	    Facing::DOWN => Facing::LEFT,
	    Facing::LEFT => Facing::UP,
	}
    }

    fn get_vector(facing: Facing) -> (i32, i32) {
	match facing {
	    Facing::UP => (-1, 0),
	    Facing::RIGHT => (0, 1),
	    Facing::DOWN => (1, 0),
	    Facing::LEFT => (0, -1),
	}
    }
}

#[derive(Debug)]
struct Guard {
    facing: Facing,
    x: i32,
    y: i32,
}

impl Guard {
    fn next(&mut self, map: &Map) {
	let next_x = self.x + Facing::get_vector(self.facing).1;
	let next_y = self.y + Facing::get_vector(self.facing).0;

	if Self::inside_map(next_x, next_y, map) && map.obstacles[next_y as usize][next_x as usize] {
	    // Hit an obstacle - turn!
	    self.facing = Facing::next(self.facing);
	}
	else {
	    self.x = next_x;
	    self.y = next_y;
	}
    }

    fn inside_map(x: i32, y: i32, map: &Map) -> bool {
	x >= 0 && (x as usize) < map.width && y >= 0 && (y as usize) < map.height
    }

    fn is_inside_map(&self, map: &Map) -> bool {
	Self::inside_map(self.x, self.y, map)
    }

    fn get_position(&self) -> (i32, i32) {
	(self.x, self.y)
    }
}

fn day6(input: &str) -> usize {

    let map: Map = Map::parse(input);
    let mut guard: Option<Guard> = None;

    print!("{:?}\n", map);
    for (y, l) in input.lines().enumerate() {
	for (x, c) in l.chars().enumerate() {
	    if c == '^' {
		let facing = Facing::UP;
		guard = Some(Guard{
		    facing,
		    x: x.try_into().unwrap(),
		    y: y.try_into().unwrap(),
		});
	    }
	}
    }

    let mut guard = guard.unwrap();
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    while guard.is_inside_map(&map) {
	print!("{:?}\n", guard);

	positions.insert(guard.get_position());
	guard.next(&map);
    }

    positions.iter().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6_example() {
	let data = std::fs::read_to_string("data/day6_example.txt").unwrap();
	let result = day6(&data);
	assert_eq!(result, 41);
    }

    #[test]
    fn day6_real() {
	let data = std::fs::read_to_string("data/day6.txt").unwrap();
	let result = day6(&data);
	assert_eq!(result, 4967);
    }
}
