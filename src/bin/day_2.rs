use advent_of_code_2024::Result;
use itertools::Itertools;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("inputs/day2.txt")?;
	let (part_1, part_2) = solve(&input)?;
	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	Ok(())
}

fn solve(input: &str) -> Result<(i64, i64)> {
	let mut safe_reports_p1 = 0;
	let mut safe_reports_p2 = 0;
	for line in input.lines() {
		let levels = line
			.split(' ')
			.filter_map(|n| n.parse::<i64>().ok())
			.collect_vec();

		if is_safe(&levels) {
			safe_reports_p1 += 1;
			safe_reports_p2 += 1;
		} else {
			let mut new_levels = Vec::with_capacity(levels.len());
			for idx in 0..levels.len() {
				new_levels.clear();
				new_levels.extend_from_slice(&levels);
				new_levels.remove(idx);
				if is_safe(&new_levels) {
					safe_reports_p2 += 1;
					break;
				}
			}
		}
	}
	Ok((safe_reports_p1, safe_reports_p2))
}

fn is_safe(levels: &[i64]) -> bool {
	let (mut positives, mut negatives) = (0, 0);

	for delta in levels.iter().tuple_windows().map(|(a, b)| b - a) {
		let abs = delta.abs();
		if delta.is_positive() && abs <= 3 {
			positives += 1;
		}
		if delta.is_negative() && abs <= 3 {
			negatives += 1;
		}
	}

	positives == levels.len() - 1 || negatives == levels.len() - 1
}

#[cfg(test)]
mod tests {
	#[test]
	fn p1_ex1() {
		let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
		let expected_p1 = 2;
		let expected_p2 = 4;
		let (p1, p2) = super::solve(input).unwrap();
		assert_eq!(p1, expected_p1);
		assert_eq!(p2, expected_p2);
	}
}
