use advent_of_code_2024::Result;
use itertools::Itertools;

fn main() -> Result<()> {
	let input = std::fs::read_to_string(concat!("inputs/", env!("CARGO_BIN_NAME"), ".txt"))?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> Result<i64> {
	let mut xmas_count = 0;

	let lines = input.lines().map(|l| l.as_bytes()).collect_vec();
	let width = lines[0].len();
	let height = lines.len();

	for j in 0..height {
		for i in 0..width {
			for buf in iter_directions(i, j, width, height, &lines) {
				if &buf == b"XMAS" {
					xmas_count += 1;
				}
			}
		}
	}

	Ok(xmas_count)
}

fn iter_directions(
	i: usize,
	j: usize,
	width: usize,
	height: usize,
	lines: &[&[u8]],
) -> impl Iterator<Item = [u8; 4]> {
	let mut words = Vec::with_capacity(8);

	if i >= 3 {
		words.push([
			lines[j][i],
			lines[j][i - 1],
			lines[j][i - 2],
			lines[j][i - 3],
		]);
	}

	if i >= 3 && j >= 3 {
		words.push([
			lines[j][i],
			lines[j - 1][i - 1],
			lines[j - 2][i - 2],
			lines[j - 3][i - 3],
		]);
	}

	if j >= 3 {
		words.push([
			lines[j][i],
			lines[j - 1][i],
			lines[j - 2][i],
			lines[j - 3][i],
		]);
	}

	if j >= 3 && i + 3 < width {
		words.push([
			lines[j][i],
			lines[j - 1][i + 1],
			lines[j - 2][i + 2],
			lines[j - 3][i + 3],
		]);
	}

	if i + 3 < width {
		words.push([
			lines[j][i],
			lines[j][i + 1],
			lines[j][i + 2],
			lines[j][i + 3],
		]);
	}

	if i + 3 < width && j + 3 < height {
		words.push([
			lines[j][i],
			lines[j + 1][i + 1],
			lines[j + 2][i + 2],
			lines[j + 3][i + 3],
		]);
	}

	if j + 3 < height {
		words.push([
			lines[j][i],
			lines[j + 1][i],
			lines[j + 2][i],
			lines[j + 3][i],
		]);
	}

	if j + 3 < height && i >= 3 {
		words.push([
			lines[j][i],
			lines[j + 1][i - 1],
			lines[j + 2][i - 2],
			lines[j + 3][i - 3],
		]);
	}

	words.into_iter()
}

fn part_2(input: &str) -> Result<i64> {
	let mut xmas_count = 0;

	let lines = input.lines().map(|l| l.as_bytes()).collect_vec();
	let width = lines[0].len();
	let height = lines.len();

	for j in 0..height {
		for i in 0..width {
			if i >= 1 && j >= 1 && i + 1 < width && j + 1 < height {
				let b: [u8; 5] = [
					lines[j - 1][i - 1],
					lines[j - 1][i + 1],
					lines[j][i],
					lines[j + 1][i - 1],
					lines[j + 1][i + 1],
				];

				if &b == b"MSAMS" || &b == b"SMASM" || &b == b"SSAMM" || &b == b"MMASS" {
					xmas_count += 1;
				}
			}
		}
	}

	Ok(xmas_count)
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
	#[test]
	fn p1_ex1() {
		let expected = 18;
		assert_eq!(super::part_1(INPUT).unwrap(), expected);
	}
	#[test]
	fn p2_ex1() {
		let expected = 9;
		assert_eq!(super::part_2(INPUT).unwrap(), expected);
	}
}
