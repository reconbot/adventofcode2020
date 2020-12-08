#[derive(Debug, PartialEq, Eq)]
enum Item {
	Empty,
	Tree
}

#[derive(Debug)]
struct Map {
	height: usize,
	width: usize,
	pattern: Vec<Vec<Item>>,
}

#[derive(Debug)]
struct Position {
	x: usize,
	y: usize
}

#[derive(Debug)]
struct Game {
	map: Map,
	position: Position,
	tree_count: i32,
}

impl Game {
	fn new(file: &str) -> Game {
		let map = Game::parse_file(file);
		Game {
			map: map,
			position: Position { x: 0, y: 0 },
			tree_count: 0,
		}
	}

	fn test(file: &str, right: usize, down: usize) -> i64 {
		let mut game = Game::new(file);
		while !game.won() {
			game.slide(right, down);
		}
		game.tree_count.into()
	}

	fn slide(&mut self, right: usize, down: usize) {
		self.position.x += right;
		self.position.y += down;
		if self.position.y >= self.map.height {
			return
		}
		if self.map.pattern[self.position.y][self.position.x % self.map.width] == Item::Tree {
			self.tree_count += 1;
		}
	}

	fn parse_file(file: &str) -> Map {
		let lines = file.lines();
		let pattern: Vec<Vec<Item>> = lines.map(|line| line.chars().map(|c| match c {
			'#' => Item::Tree,
			_ => Item::Empty
		}).collect()).collect();

		Map {
			height: pattern.len(),
			width: pattern[0].len(),
			pattern
		}
	}

	fn won(&self) -> bool {
		self.position.y >= self.map.height
	}
}

fn day3a() {
	let file = include_str!("./day3.txt");
	println!("1,1  hit {} trees", Game::test(file, 3, 1));
	println!{"{:?}",[
		Game::test(file, 1, 1) *
		Game::test(file, 3, 1) *
		Game::test(file, 5, 1) *
		Game::test(file, 7, 1) *
		Game::test(file, 1, 2)]
	}
}

fn main() {
  day3a();
}
