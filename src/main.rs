use anyhow::{Context, Result};
use dashmap::DashMap;
use gxhash::{gxhash128, GxHasher};
use rayon::prelude::*;
use std::fs::File;
use std::hash::Hasher;
use std::io::Read;
use std::{env, fs};

const SEED: i64 = 109832;

#[derive(Clone)]
struct Item {
	path: String,
	is_file: bool,
	is_folder: bool,
	children: Option<Vec<Item>>,
	state: Option<u128>,
}

fn compute_checksum(input: &[u8]) -> u128 {
	gxhash128(input, SEED)
}

fn get_file_checksum(path: &str) -> Result<u128> {
	let mut file = File::open(path).context("Failed to open file")?;
	let mut hasher = GxHasher::with_seed(SEED);
	let mut buffer = [0; 65536];

	while let Ok(bytes_read) = file.read(&mut buffer) {
		if bytes_read == 0 {
			break;
		}
		hasher.write(&buffer[..bytes_read]);
	}

	Ok(hasher.finish_u128())
}

fn get_state(item: &Item) -> Result<u128> {
	if item.is_file {
		get_file_checksum(&item.path)
	}
	else if let Some(children) = &item.children {
		compute_children_checksum(children)
	}
	else {
		Err(anyhow::anyhow!("Unexpected item type"))
	}
}

fn compute_children_checksum(children: &[Item]) -> Result<u128> {
	let state_bytes: Vec<u8> = children
		.par_iter()
		.filter_map(|child| get_state(child).ok().map(|state| state.to_be_bytes()))
		.flatten()
		.collect();

	Ok(compute_checksum(&state_bytes))
}

fn get_item(
	path: &str,
	items: &DashMap<String, Item>,
	seen_states: &DashMap<u128, String>
) -> Result<Item> {
	if let Some(item) = items.get(path) {
		return Ok(item.clone());
	}

	let metadata = fs::metadata(path)
		.with_context(|| format!("Failed to read metadata for path: {}", path))?;
	let is_file = metadata.is_file();
	let is_folder = metadata.is_dir();
	let children = if is_folder {
		Some(get_children(path, items, seen_states)?)
	}
	else {
		None
	};

	let item = Item {
		path: path.to_string(),
		is_file,
		is_folder,
		children,
		state: None,
	};

	let state = get_state(&item)?;
	let mut item = item;
	item.state = Some(state);

	if let Some(existing_path) = seen_states.get(&state) {
		if item.is_folder {
			println!("Removing: {}", path);
			// Handle the directory removal with proper error handling
			// fs::remove_dir_all(path).with_context(|| format!("Failed to remove directory: {}", path))?;
		}
	}
	else {
		seen_states.insert(state, path.to_string());
	}

	items.insert(path.to_string(), item.clone());

	Ok(item)
}

fn get_children(
	path: &str,
	items: &DashMap<String, Item>,
	seen_states: &DashMap<u128, String>
) -> Result<Vec<Item>> {
	fs::read_dir(path)
		.with_context(|| format!("Failed to read directory: {}", path))?
		.par_bridge()
		.filter_map(Result::ok)
		.map(|entry| {
			let path_str = entry.path().to_string_lossy().to_string();
			get_item(&path_str, items, seen_states)
				.with_context(|| format!("Failed to get item for path: {}", path_str))
		})
		.collect()
}

fn main() -> Result<()> {
	let root_path = env::args().nth(1).context("Please provide the root path as an argument")?;

	let items = DashMap::new();
	let seen_states = DashMap::new();
	let _root_item = get_item(&root_path, &items, &seen_states)?;

	Ok(())
}
