use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
struct Bag<'a> {
	name: String,
	parents: HashSet<Quantity<&'a Bag<'a>>>,
	children: HashSet<Quantity<&'a Bag<'a>>>
}

impl <'a> Hash for Bag<'a> {
	fn hash<H: Hasher>(&self, state: &mut H) {
			self.name.hash(state);
	}
}

impl <'a> PartialEq for Bag<'a> {
	fn eq(&self, other: &Self) -> bool {
			self.name == other.name
	}
}

impl <'a> Bag<'a> {
	fn new(name: String) -> Self {
		Self {
			name: name,
			parents: HashSet::new(),
			children: HashSet::new(),
		}
	}
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Quantity<T> {
	number: i32,
	item: T,
}

struct BagGraph<'a> {
	map: HashMap<String, Bag<'a>>,
}

impl <'a> BagGraph<'a> {
	fn new() -> Self {
		Self {
			map: HashMap::new()
		}
	}

	fn add(self: &mut Self, parent_name: &str, contains: i32, child_name: &str) {
		let parent = self.map.entry(parent_name.to_string()).or_insert(Bag::new(parent_name.to_string()));
		let child = self.map.entry(child_name.to_string()).or_insert(Bag::new(child_name.to_string()));
		parent.children.insert(Quantity{
			number: contains,
			item: child,
		});
		child.parents.insert(Quantity{
			number: contains,
			item: parent,
		});
	}
}

fn main() {
	let file = include_str!("./day7.txt");
	let mut bags= BagGraph::new();
	// dotted tomato bags contain 3 dotted maroon bags.
	bags.add("dotted tomato", 3, "dotted maroon");
	// posh purple bags contain 3 drab turquoise bags, 3 dark olive bags, 4 posh lime bags, 1 posh orange bag.
	// vibrant beige bags contain 2 mirrored violet bags, 1 mirrored white bag, 1 wavy violet bag.
}
