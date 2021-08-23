use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct BagRef {
	bag: Weak<Bag>
}

impl BagRef {
	fn new(bag: Weak<Bag>) -> Self {
		Self {
			bag: bag
		}
	}}

impl PartialEq for BagRef {
	fn eq(&self, other: &Self) -> bool {
		self.bag.upgrade().unwrap() == other.bag.upgrade().unwrap()
	}
}

impl Eq for BagRef {}

impl Hash for BagRef {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.bag.upgrade().unwrap().hash(state);
	}
}

#[derive(Debug, Eq)]
struct Bag {
	name: String,
	parents: HashSet<BagRef>,
	children: HashSet<BagRef>
}

impl Hash for Bag {
	fn hash<H: Hasher>(&self, state: &mut H) {
			self.name.hash(state);
	}
}

impl PartialEq for Bag {
	fn eq(&self, other: &Self) -> bool {
			self.name == other.name
	}
}

impl Bag {
	fn new(name: String) -> Self {
		Self {
			name: name,
			parents: HashSet::new(),
			children: HashSet::new(),
		}
	}
}


#[derive(Debug)]
struct BagGraph {
	map: HashMap<String, Rc<RefCell<Bag>>>,
}

impl BagGraph {
	fn new() -> Self {
		Self {
			map: HashMap::new()
		}
	}

	fn add(self: &mut Self, parent_name: &str, child_name: &str) {
		if !self.map.contains_key(parent_name) {
			self.map.insert(parent_name.to_string(), Rc::new(RefCell::new(Bag::new(parent_name.to_string()))));
		}
		if !self.map.contains_key(child_name) {
			self.map.insert(child_name.to_string(), Rc::new(RefCell::new(Bag::new(child_name.to_string()))));
		}
		let parent = self.map.get(parent_name).unwrap();
		let child = self.map.get(child_name).unwrap();

		let mut children = parent.borrow_mut().children;
		let mut parents = parent.borrow_mut().parents;

		children.insert(BagRef::new(Rc::<Bag>::downgrade(child)));
		parents.insert(BagRef::new(Rc::<Bag>::downgrade(parent)));
	}
}

fn main() {
	let file = include_str!("./day7.txt");
	let mut bags = BagGraph::new();
	// dotted tomato bags contain 3 dotted maroon bags.
	bags.add("dotted tomato", "dotted maroon");
	bags.add("posh purple bags", "drab turquoise bags");
	bags.add("posh purple bags", "dark olive bags");
	bags.add("posh purple bags", "dotted tomato");

	println!("{:?}", bags);
}
