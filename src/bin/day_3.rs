use advent_of_code_2024::Result;

fn main() -> Result<()> {
	let input = std::fs::read_to_string("inputs/day3.txt")?;
	println!("Part 1: {}", part_1(&input)?);
	println!("Part 2: {}", part_2(&input)?);
	Ok(())
}

fn part_1(input: &str) -> Result<i64> {
	let r = regex::Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#)?;
	let mut sum = 0;
	for cap in r.captures_iter(input) {
		let a = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
		let b = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
		sum += a * b;
	}
	Ok(sum)
}

fn part_2(input: &str) -> Result<i64> {
	let r = regex::Regex::new(r#"(mul)\((\d{1,3}),(\d{1,3})\)|(do)\(\)|(don't)\(\)"#)?;
	let mut sum = 0;
	let mut mul_enabled = true;
	for cap in r.captures_iter(input) {
		if cap.get(4).is_some() {
			mul_enabled = true;
			continue;
		}
		if cap.get(5).is_some() {
			mul_enabled = false;
			continue;
		}
		if cap.get(1).is_some() && mul_enabled {
			let a = cap.get(2).unwrap().as_str().parse::<i64>().unwrap();
			let b = cap.get(3).unwrap().as_str().parse::<i64>().unwrap();
			sum += a * b;
		}
	}
	Ok(sum)
}

#[cfg(test)]
mod tests {
	#[test]
	fn p1_ex1() {
		let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
		let expected = 161;
		let result = super::part_1(input).unwrap();
		assert_eq!(result, expected);
	}

	#[test]
	fn p2_ex_1() {
		let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
		let expected = 48;
		let result = super::part_2(input).unwrap();
		assert_eq!(result, expected);
	}
}
