use crate::data::*;
use std::collections::HashMap;

fn parse_key_value(data: &str) -> Option<HashMap<u8, String>> {
	let split: Vec<_> = data.split(':').collect();
	let mut parsed = HashMap::new();

	for chunk in split.chunks(2) {
		parsed.insert(chunk[0].parse().ok()?, chunk[1].into());
	}

	Some(parsed)
}

pub trait APIData: Sized {
	fn parse_data(data: &str) -> Option<Self>;
}

impl APIData for Level {
	fn parse_data(data: &str) -> Option<Self> {
		let mut map = parse_key_value(data)?;

		let id = map.get(&1)?.parse().ok()?;
		let name = map.remove(&2)?;

		Some(Self { id, name })
	}
}

impl APIData for User {
	fn parse_data(data: &str) -> Option<Self> {
		let mut map = parse_key_value(data)?;

		let name = map.remove(&1)?;
		let id = map.get(&16)?.parse().ok()?;

		Some(Self { id, name })
	}
}
