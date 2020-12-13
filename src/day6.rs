use std::collections::HashSet;
use std::collections::HashMap;

fn main6a() {
	let file = include_str!("./day6.txt");
	let count = file.split("\n\n").map(|group| {
		let mut set = HashSet::new();
		dbg!(group);
		for c in group.chars() {
			set.insert(c.to_string());
		}
		set.remove(&'\n'.to_string());
		set.len()
	}).fold(0, |acc, x| acc+x );

	println!("counted {}", count);
}


fn main() {
	let file = include_str!("./day6.txt");
	let count = file.split("\n\n").map(|group| {
		let group = group.trim();
		let mut map = HashMap::new();
		for c in group.chars() {
			let counter = map.entry(c).or_insert(0);
			*counter += 1;
		}
		let person_count = *(map.entry('\n').or_insert(0)) + 1;
		dbg!(group, &map, &person_count);
		for (key, val) in map.clone().iter() {
			if val < &person_count {
				map.remove(key);
			}
		}
		dbg!(group, &map);
		map.len()
	}).fold(0, |acc, x| acc+x );

	println!("counted {}", count);
}
