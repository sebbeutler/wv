#![allow(unused)]

use std::rc::Rc;

use crate::schema::{SchemaObject, SchemaValue};

#[derive(Debug, Clone, PartialEq)]
pub enum Script {
	Python(String),
	JavaScript(String),
	C,
	WGSL,
}

pub struct Node {
	pronode: Option<Box<Self>>,
	subnodes: Vec<Node>,
	name: Box<str>,
	hash: usize,
	schema: SchemaObject,
	script: Option<Script>,
}

impl Node {
	pub fn new(name: &str) -> Self {
		Node {
			pronode: None,
			subnodes: Vec::new(),
			name: name.into(),
			hash: 0,
			schema: SchemaObject::new(),
			script: None,
		}
	}

	pub fn properties(&self) -> &SchemaObject {
		&self.schema
	}

	pub fn with_schematic(mut self, props: &SchemaObject) -> Self {
		self.schema.update(props);
		self
	}

	pub fn pronode(&self) -> Option<&Box<Self>> {
		self.pronode.as_ref()
	}

	pub fn subnodes(&self) -> &Vec<Node> {
		&self.subnodes
	}

	pub fn add_node(&mut self, node: Node) {
		self.subnodes.push(node);
	}

	pub fn find_node(&self, name: &str) -> Option<&Node> {
		if self.name == name.into() {
			return Some(self);
		}
		for node in &self.subnodes {
			if let Some(found) = node.find_node(name) {
				return Some(found);
			}
		}
		None
	}

	pub fn find_subnodes(&self, name: &str) -> Vec<&Node> {
		let mut nodes = Vec::new();
		for node in &self.subnodes {
			if node.name == name.into() {
				nodes.push(node);
			}
			nodes.extend(node.find_subnodes(name));
		}
		nodes
	}

	pub fn set_script(mut self, source: Script) -> Self {
		self.script = Some(source);
		self
	}
}

pub struct BoardState {
	root_node: Node,
}

pub type BoardStateMutex = std::sync::Mutex<BoardState>;

impl BoardState {
	pub fn add_node(&mut self, node: Node) {
		self.root_node.add_node(node);
	}

	pub fn properties(&self) -> &SchemaObject {
		&self.root_node.properties()
	}

	pub fn as_mutex(self) -> std::sync::Mutex<Self> {
		std::sync::Mutex::new(self)
	}
}

pub fn create_board() -> BoardState {
	let root_node = Node::new("root").with_schematic(&schema!
	({
		name: "MyBoard",
		PYTHON_PATH: "",
		cwd: "",
	}));
	BoardState { root_node }
}
