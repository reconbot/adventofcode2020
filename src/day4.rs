#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::collections::HashSet;

lazy_static! {
	static ref PASSPORT_FIELD_SPLIT_RE: Regex = Regex::new("\\s+").unwrap();
}

lazy_static! {
	static ref PASSPORT_FIELD_PAIR_RE: Regex = Regex::new("(\\S+):(\\S+)").unwrap();
}

#[derive(Debug)]
struct Passport<'a> {
	byr: &'a str, // (Birth Year)
	iyr: &'a str, // (Issue Year)
	eyr: &'a str, // (Expiration Year)
	hgt: &'a str, // (Height)
	hcl: &'a str, // (Hair Color)
	ecl: &'a str, // (Eye Color)
	pid: &'a str, // (Passport ID)
	cid: &'a str, // (Country ID)
}

lazy_static! {
	static ref REQUIRED_FIELDS: [&'static str; 7] = [
		"byr", // (Birth Year)
		"iyr", // (Issue Year)
		"eyr", // (Expiration Year)
		"hgt", // (Height)
		"hcl", // (Hair Color)
		"ecl", // (Eye Color)
		"pid", // (Passport ID)
	];
}

#[derive(Debug)]
struct Customs;
// {
// 	valid_passports: Vec<Passport>,
// }

impl Customs {
	// fn new(file: &str) -> Self {
	// 	Self {
	// 		valid_passports: Self::parse_file(file)
	// 	}
	// }

	fn valid_value(key: &str, value: &str) -> bool {
		match key {
			"byr"  =>  { // (Birth Year) - four digits; at least 1920 and at most 2002.
				if Regex::new("^\\d{4}$").unwrap().is_match(value) {
					let date = value.parse::<i32>().unwrap();
					date >= 1920 && date <= 2002
				} else {
					false
				}
			},
			"iyr"  =>  { // (Issue Year) - four digits; at least 2010 and at most 2020.
				if Regex::new("^\\d{4}$").unwrap().is_match(value) {
					let date = value.parse::<i32>().unwrap();
					date >= 2010 && date <= 2020
				} else {
					false
				}
			},
			"eyr"  =>  { // Expiration Year) - four digits; at least 2020 and at most 2030.
				if Regex::new("^\\d{4}$").unwrap().is_match(value) {
					let date = value.parse::<i32>().unwrap();
					date >= 2020 && date <= 2030
				} else {
					false
				}
			},
			"hgt" => {  // (Height) - a number followed by either cm or in:
				match Regex::new("^(\\d+)(cm|in)$").unwrap().captures(value) {
					Some(caps) => {
						let val = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
						match caps.get(2).unwrap().as_str() {
							//If cm, the number must be at least 150 and at most 193.
							"cm" => val >= 150 && val <= 193,
							//If in, the number must be at least 59 and at most 76.
							"in" => val >= 59 && val <= 76,
							_ => false,
						}
					},
					None => false
				}
			},
			"hcl" => { // (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
				Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(value)
			},
			"ecl" => { // (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
				Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap().is_match(value)
			},
			"pid" => { // (Passport ID) - a nine-digit number, including leading zeroes.
				Regex::new("^\\d{9}$").unwrap().is_match(value)
			},
			_ => true
		}
	}

	fn parse_file(file: &str) -> i32 {
		let mut valid_passports = 0;
		let raw_passports = file.split("\n\n");
		for raw_passport in raw_passports {
			let mut missing_fields: HashSet<&'static str> = REQUIRED_FIELDS.iter().cloned().collect();
			for pair in PASSPORT_FIELD_SPLIT_RE.split(raw_passport.trim()) {
				let matches = PASSPORT_FIELD_PAIR_RE.captures(pair).unwrap();
				let key = &matches[1];
				let value = &matches[2];
				if Self::valid_value(key, value) {
					println!("{:?}={:?} is valid", key, value);
					missing_fields.remove(key);
				}
			}
			if missing_fields.len() == 0 {
				valid_passports+=1;
				// dbg!(raw_passport);
			}
		};
		valid_passports
	}

}

fn day4a() {
	let file = include_str!("./day4.txt");
  let customs = Customs::parse_file(file);
  println!("found {:?} valid passports", customs);
}

fn main() {
  day4a();
}
