#[derive(Debug,PartialEq,Eq,Clone)]
struct Seat{
	row: i32,
	col: i32,
}

impl Seat {
	fn new(row: i32, col: i32) -> Self {
		Seat {
			row: row,
			col: col,
		}
	}

	fn id(&self) -> i32 {
		(self.row * 8) + self.col
	}

	fn from_code(code: &str) -> Option<Self> {
		if code.chars().count() < 10 {
			return None;
		}
		let row_code = &code[0..7];
		let col_code = &code[7..];
		let mut row = (0, 127);
		let mut col = (0,7);
		for c in row_code.chars() {
			match c {
				'F' => row = (row.0, ((row.0 + row.1 + 1) / 2)),
				'B' => row = (((row.0 + row.1 + 1) / 2) , row.1),
				_ => {}
			}
		}
		for c in col_code.chars() {
			match c {
				'L' => col = (col.0, ((col.0 + col.1 + 1) / 2)),
				'R' => col = (((col.0 + col.1 + 1) / 2), col.1),
				_ => {}
			}
		}
		// dbg!(row_code, row, col_code, col);
		Some(Seat {
			row: row.0,
			col: col.0,
		})
	}
}

#[test]
fn id_test() {
	let seat = Seat::new(44, 5);
	assert_eq!(seat.id(), 357);
}


#[test]
fn from_code_test() {
	assert_eq!(Seat::from_code("BFFFBBFRRR"), Seat { row: 70, col: 7 });
	assert_eq!(Seat::from_code("FFFBBBFRRR"), Seat { row: 14, col: 7 });
	assert_eq!(Seat::from_code("BBFFBBFRLL"), Seat { row: 102, col: 4 });
}


fn parse_file(file: &str) -> Vec<Seat> {
	file.trim().lines().filter_map(|line| Seat::from_code(line)).collect()
}

fn main() {
	let file = include_str!("./day5.txt");
	let seats = parse_file(file);
	let highest_id = seats.iter().map(|seat| seat.id()).fold(0, |acc, x| if x > acc { x } else { acc });
	println!("{:?}", highest_id);
	let mut sorted_seats: Vec<i32> = seats.iter().map(|seat| seat.id()).collect();
	sorted_seats.sort();
	// dbg!(sorted_seats);
	for (index, seat) in sorted_seats.iter().enumerate() {
		if index == 0 {
			continue;
		}
		if sorted_seats[index + 1] != (seat + 1) {
			println!("Your seat id is {}", seat + 1);
			return;
		}
	}
}
