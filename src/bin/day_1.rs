use itertools::Itertools;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("inputs/day1.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> Result<i64> {
	let (mut left, mut right): (Vec<i64>, Vec<i64>) = parse(input)?;
	left.sort();
	right.sort();

	Ok(left
		.into_iter()
		.zip(right)
		.map(|(left, right)| (left - right).abs())
		.sum::<i64>())
}

fn part_2(input: &str) -> Result<i64> {
	let (left, right): (Vec<i64>, Vec<i64>) = parse(input)?;

	Ok(left
		.iter()
		.copied()
		.map(|l| l * right.iter().copied().filter(|r| *r == l).count() as i64)
		.sum::<i64>())
}

fn parse(input: &str) -> Result<(Vec<i64>, Vec<i64>)> {
	input
		.lines()
		.flat_map(|line| line.split_ascii_whitespace())
		.tuples()
		.try_fold(
			(Vec::new(), Vec::new()),
			|(mut left_list, mut right_list), (left, right)| -> Result<_> {
				left_list.push(left.parse::<i64>()?);
				right_list.push(right.parse::<i64>()?);
				Ok((left_list, right_list))
			},
		)
}

#[cfg(test)]
mod tests {
	#[test]
	fn p1_ex1() {
		let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
		let expected = 11;
		assert_eq!(super::part_1(input).unwrap(), expected);
	}

	#[test]
	fn p2_ex1() {
		let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
		let expected = 31;
		assert_eq!(super::part_2(input).unwrap(), expected);
	}
}
