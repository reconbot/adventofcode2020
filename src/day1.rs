fn parse_entries(file: &str) -> Vec<i32> {
	let mut entries = vec![];
	for line in file.lines() {
		entries.push(line.parse::<i32>().unwrap())
	}

	entries
}

fn day1() {
	let entries = include_str!("./day1.txt");
	let amounts = parse_entries(entries);
	let total = 2020;
	'outer: for a in &amounts {
		for b in &amounts {
			if a + b == total {
				println!(
					"FOUND!\n{} + {} = {}\n{} * {} = {}",
					a,
					b,
					total,
					a,
					b,
					a*b
				);
				break 'outer;
			}
		}
	}
}

fn day1b() {
	let entries = include_str!("./day1.txt");
	let amounts = parse_entries(entries);
	let total = 2020;
	'outer: for a in &amounts {
		for b in &amounts {
      for c in &amounts {
        if a + b + c == total {
          println!(
            "FOUND!\n{} + {} + {} = {}\n{} * {} * {} = {}",
            a,
            b,
            c,
            total,
            a,
            b,
            c,
            a*b*c
          );
          break 'outer;
        }
      }
    }
	}
}

fn main() {
  day1();
  day1b();
}
