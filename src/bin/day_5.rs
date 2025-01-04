use advent_of_code_2024::Result;
use itertools::Itertools;

fn main() -> Result<()> {
	let input = std::fs::read_to_string(concat!("inputs/", env!("CARGO_BIN_NAME"), ".txt"))?;
	let Answer { part_1, part_2 } = solve(&input)?;
	println!("Part 1: {part_1}");
	println!("Part 2: {part_2}");
	Ok(())
}

struct Answer {
	part_1: i64,
	part_2: i64,
}

fn solve(input: &str) -> Result<Answer> {
	let (rules, updates) = input.split("\n\n").next_tuple().unwrap();
	let rules: Vec<(i64, i64)> = rules
		.lines()
		.map(|l| {
			l.split('|')
				.map(|n| n.parse().unwrap())
				.next_tuple()
				.unwrap()
		})
		.collect();
	let updates: Vec<Vec<i64>> = updates
		.lines()
		.map(|l| l.split(',').map(str::parse).try_collect())
		.try_collect()?;

	let mut part_1 = 0;
	let mut incorrect_updates: Vec<Vec<i64>> = Vec::new();

	'update_check: for update in updates {
		for i in 0..update.len() {
			let x = update[i];

			for ii in i..update.len() {
				let y = update[ii];

				for (before, after) in rules.iter().copied() {
					if x == after && y == before {
						incorrect_updates.push(update);
						continue 'update_check;
					}
				}
			}
		}

		part_1 += update[update.len() / 2];
	}

	let mut part_2 = 0;

	for mut update in incorrect_updates {
		let middle_idx = update.len() / 2;
		let mut last = 0;

		while update.len() != middle_idx {
			let (idx, _) = update
				.iter()
				.find_position(|&&u| {
					!rules
						.iter()
						.any(|(before, after)| *after == u && update.contains(before))
				})
				.unwrap();
			last = update.remove(idx);
		}

		part_2 += last;
	}

	Ok(Answer { part_1, part_2 })
}

#[cfg(test)]
mod tests {
	const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

	#[test]
	fn p1_ex1() {
		let expected = 143;
		let answer = super::solve(INPUT).unwrap();
		assert_eq!(answer.part_1, expected);
	}

	#[test]
	fn p2_ex1() {
		let expected = 123;
		let answer = super::solve(INPUT).unwrap();
		assert_eq!(answer.part_2, expected);
	}
}
