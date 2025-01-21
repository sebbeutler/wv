#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SchemaValue {
	String(String),
	U64(u64),
	I64(i64),
	Bool(bool),
	SchemaLink(String),
	SchemaObject(SchemaObject),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaObject {
	pub entries: HashMap<String, SchemaValue>,
}

impl SchemaObject {
	pub fn new() -> Self {
		Self {
			entries: HashMap::new(),
		}
	}

	pub fn get(&self, key: &str) -> Option<&SchemaValue> {
		self.entries.get(key)
	}

	pub fn update(&mut self, other: &SchemaObject) {
		for (k, v) in &other.entries {
			self.entries.insert(k.clone(), v.clone());
		}
	}
}

impl From<&str> for SchemaValue {
	fn from(s: &str) -> Self {
		SchemaValue::String(s.to_string())
	}
}

impl From<String> for SchemaValue {
	fn from(s: String) -> Self {
		SchemaValue::String(s)
	}
}

impl From<u64> for SchemaValue {
	fn from(n: u64) -> Self {
		SchemaValue::U64(n)
	}
}

impl From<i64> for SchemaValue {
	fn from(n: i64) -> Self {
		SchemaValue::I64(n)
	}
}

impl From<bool> for SchemaValue {
	fn from(b: bool) -> Self {
		SchemaValue::Bool(b)
	}
}

#[macro_export]
macro_rules! schema {
    // --- Base case: no more pairs ---
    ({}) => {{
        SchemaObject::new()
    }};

    // --- Case 1: The next pair is a block `{ ... }`, and there's more after it ---
    ({ $key:ident : { $($block_contents:tt)* }, $($rest:tt)* }) => {{
        // Recursively parse the rest first
        let mut obj = schema!({ $($rest)* });
        // Then insert the current block value
        schema_item!(obj; $key : { $($block_contents)* });
        obj
    }};

    // --- Case 2: The next pair is an expression, and there's more after it ---
    ({ $key:ident : $val:expr, $($rest:tt)* }) => {{
        // Recursively parse the rest first
        let mut obj = schema!({ $($rest)* });
        // Then insert the current expression
        schema_item!(obj; $key : $val);
        obj
    }};

    // --- Case 3: Single last pair is a block (no trailing comma) ---
    ({ $key:ident : { $($block_contents:tt)* } }) => {{
        let mut obj = SchemaObject::new();
        schema_item!(obj; $key : { $($block_contents)* });
        obj
    }};

    // --- Case 4: Single last pair is an expression (no trailing comma) ---
    ({ $key:ident : $val:expr }) => {{
        let mut obj = SchemaObject::new();
        schema_item!(obj; $key : $val);
        obj
    }};
}

#[macro_export]
macro_rules! schema_item {
    // If the value is a block
    ($obj:ident; $key:ident : { $($inner:tt)* }) => {
        $obj.entries.insert(
            stringify!($key).to_string(),
            SchemaValue::SchemaObject(schema!({ $($inner)* })),
        );
    };

    // If the value is an expression
    ($obj:ident; $key:ident : $val:expr) => {
        let computed_value = $val;
        $obj.entries.insert(
            stringify!($key).to_string(),
            SchemaValue::from(computed_value),
        );
    };
}

pub fn link(s: &str) -> SchemaValue {
	SchemaValue::SchemaLink(s.to_string())
}

#[cfg(test)]
mod test_schema {
	use super::*;

	#[test]
	fn test_schema_macro() {
		let obj = schema!({
			link: String::from("test"),
		});

		assert_eq!(
			obj.get("link"),
			Some(&SchemaValue::String("test".to_string()))
		);

		let mut obj = schema!({
			name: "Rust",
			stars: 100_000u64,
			year: 2015i64,
			nested: {
				active: true,
			},
		});

		assert_eq!(
			obj.get("name"),
			Some(&SchemaValue::String("Rust".to_string()))
		);
		assert_eq!(obj.get("stars"), Some(&SchemaValue::U64(100_000)));
		assert_eq!(obj.get("year"), Some(&SchemaValue::I64(2015)));
		assert_eq!(
			obj.get("nested"),
			Some(&SchemaValue::SchemaObject(SchemaObject {
				entries: {
					let mut nested = HashMap::new();
					nested.insert("active".to_string(), SchemaValue::Bool(true));
					nested
				}
			}))
		);

		let update_obj = schema!({
			name: "Rust Programming Language",
			version: "1.70.0",
		});
		obj.update(&update_obj);

		assert_eq!(
			obj.get("name"),
			Some(&SchemaValue::String(
				"Rust Programming Language".to_string()
			))
		);
		assert_eq!(
			obj.get("version"),
			Some(&SchemaValue::String("1.70.0".to_string()))
		);
		assert_eq!(obj.get("stars"), Some(&SchemaValue::U64(100_000)));

		let empty_obj = schema!({});
		assert_eq!(empty_obj.entries.len(), 0);
	}
}
