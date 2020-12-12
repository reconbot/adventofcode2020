#[derive(Debug,PartialEq,Eq)]
struct Seat {
	row: i32,
	col: i32,
}

impl Seat {
	fn id(&self) -> i32 {
		(self.row * 8) + self.col
	}

	fn from_code(code: &str) -> Self {
		let row_code = &code[0..7];
		let col_code = &code[7..];
		let mut row = 127;
		let mut col = 7;
		for c in row_code.chars() {
			match c {
				'F' => row = row >> 1,
				'B' => row = row >> 1 << 1,
				_ => {}
			}
		}
		for c in col_code.chars() {
			match c {
				'R' => col = col << 1,
				'L' => col = col >> 1,
				_ => {}
			}
		}
		dbg!(row_code, row, col_code, col);
		Seat {
			row: row,
			col: col,
		}
	}
}

#[test]
fn id_test() {
	let seat = Seat { row: 44, col: 5 };
	assert_eq!(seat.id(), 357);
}


#[test]
fn from_code_test() {
	assert_eq!(Seat::from_code("BFFFBBFRRR"), Seat { row: 70, col: 7 });
	assert_eq!(Seat::from_code("FFFBBBFRRR"), Seat { row: 14, col: 7 });
	assert_eq!(Seat::from_code("BBFFBBFRLL"), Seat { row: 102, col: 4 });
}



fn parse_file(file: &str) {

}

fn main() {
	let file = include_str!("./day4.txt");
  parse_file(file);

}
