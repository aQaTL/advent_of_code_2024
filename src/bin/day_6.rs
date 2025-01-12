use advent_of_code_2024::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
	let input = std::fs::read_to_string(concat!("inputs/", env!("CARGO_BIN_NAME"), ".txt"))?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

#[derive(Default, Copy, Clone, Eq, Hash, PartialEq)]
struct Pos {
	x: i64,
	y: i64,
}

const DIRECTION_UP: Pos = Pos { x: 0, y: -1 };
const DIRECTION_RIGHT: Pos = Pos { x: 1, y: 0 };
const DIRECTION_DOWN: Pos = Pos { x: 0, y: 1 };
const DIRECTION_LEFT: Pos = Pos { x: -1, y: 0 };

const DIRECTIONS: [Pos; 4] = [
	DIRECTION_UP,
	DIRECTION_RIGHT,
	DIRECTION_DOWN,
	DIRECTION_LEFT,
];

impl std::ops::Add for Pos {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Pos {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

fn part_1(input: &str) -> Result<usize> {
	let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

	let map_ref: &Vec<_> = map.as_ref();
	let mut guard_pos = find_guard_pos::<&Vec<_>, &&[u8]>(map_ref)?;
	let mut guard_dir_idx = 0;

	let mut positions = HashSet::new();
	positions.insert(guard_pos);

	loop {
		let new_pos = guard_pos + DIRECTIONS[guard_dir_idx];
		if !position_valid(&map, new_pos) {
			break;
		}
		if map[new_pos.y as usize][new_pos.x as usize] == b'#' {
			guard_dir_idx = (guard_dir_idx + 1) % DIRECTIONS.len();
		} else {
			guard_pos = new_pos;
			positions.insert(guard_pos);
		}
	}

	Ok(positions.len())
}

fn find_guard_pos<I, C>(map: I) -> Result<Pos>
where
	I: IntoIterator<Item = C>,
	C: AsRef<[u8]>,
{
	for (j, row) in map.into_iter().enumerate() {
		for (i, cell) in row.as_ref().iter().enumerate() {
			if *cell == b'^' {
				return Ok(Pos {
					x: i as i64,
					y: j as i64,
				});
			}
		}
	}
	Err("guard pos not found".into())
}

fn position_valid<T, U>(map: &[T], pos: Pos) -> bool
where
	T: AsRef<[U]>,
{
	if pos.x < 0 && pos.y < 0 {
		return false;
	}
	map.get(pos.y as usize)
		.and_then(|row| row.as_ref().get(pos.x as usize))
		.is_some()
}

fn part_2(input: &str) -> Result<usize> {
	let map: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

	let mut loops_detected = 0;
	let mut positions = HashSet::<(Pos, usize)>::new(); // (position, direction)

	for (y, row) in map.iter().enumerate() {
		for (x, cell) in row.iter().enumerate() {
			if *cell == b'#' || *cell == b'^' {
				continue;
			}

			let mut guard_pos = find_guard_pos(&map)?;
			let mut guard_dir_idx = 0;

			positions.clear();
			positions.insert((guard_pos, guard_dir_idx));

			loop {
				let new_pos = guard_pos + DIRECTIONS[guard_dir_idx];
				if !position_valid(&map, new_pos) {
					break;
				}
				if map[new_pos.y as usize][new_pos.x as usize] == b'#'
					|| new_pos.x as usize == x && new_pos.y as usize == y
				{
					guard_dir_idx = (guard_dir_idx + 1) % DIRECTIONS.len();
				} else {
					guard_pos = new_pos;
				}
				let new_value = positions.insert((guard_pos, guard_dir_idx));
				if !new_value {
					loops_detected += 1;
					break;
				}
			}
		}
	}

	Ok(loops_detected)
}

#[cfg(test)]
mod tests {
	const EX1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

	#[test]
	fn p1_ex1() {
		let expected = 41;
		let result = super::part_1(EX1).unwrap();
		assert_eq!(result, expected);
	}

	#[test]
	fn p2_ex1() {
		let expected = 6;
		let result = super::part_2(EX1).unwrap();
		assert_eq!(result, expected);
	}
}
