#[derive(Debug)]
struct Password<'a> {
	min: i32,
	max: i32,
	letter: char,
	password: &'a str,
}

fn parse_entries(file: &str) -> Vec<Password> {
	let mut entries = vec![];
	for line in file.lines() {
		let mut parts = line.split(" ");
		let mut range = parts.next().unwrap().split("-");
		let min = range.next().unwrap().parse::<i32>().unwrap();
		let max = range.next().unwrap().parse::<i32>().unwrap();

		let letter = parts.next().unwrap().chars().next().unwrap();
		let password = parts.next().unwrap();

		let pwd = Password {
			min: min,
			max: max,
			letter: letter,
			password: password,
		};
		entries.push(pwd);
	}

	entries
}

fn day2a() {
	let entries = include_str!("./day2.txt");
	let passwords = parse_entries(entries);
	let mut valid_count = 0;
	let mut total_count = 0;
	for entry in passwords {
		total_count += 1;
		let count = entry.password.chars().filter(|c| *c == entry.letter).count();
		if count >= (entry.min as usize) && count <= (entry.max as usize) {
			valid_count+= 1;
		}
	}
	println!("There are {:?} valid passwords of {} total passwords", valid_count, total_count);
}

fn day2b() {
	let entries = include_str!("./day2.txt");
	let passwords = parse_entries(entries);
	let mut valid_count = 0;
	let mut total_count = 0;
	for entry in passwords {
		total_count += 1;
		let chars: Vec<char> = entry.password.chars().collect();
		let pos1 = match chars.get(entry.min as usize - 1) {
			Some(c) => *c == entry.letter,
			None => false
		};

		let pos2 = match chars.get(entry.max as usize - 1) {
			Some(c) => *c == entry.letter,
			None => false
		};

		if (pos1 && !pos2 )|| (!pos1 && pos2) {
			print!("VALID MATCH! {} ^ {} {}-{} {}: ", pos1, pos2, entry.min, entry.max, entry.letter);
			println!("{:?}", entry.password.chars().enumerate().map(|(i, c)| (i+1, c)).collect::<Vec<(usize,char)>>());
			valid_count+=1;
		}
	}
	println!("There are {:?} valid passwords of {} total passwords", valid_count, total_count);
}

fn main() {
  day2b();
}
