use anyhow::{Context, Result};
use gxhash::{gxhash128, HashMap, HashMapExt};
use std::{collections::HashSet, env, fs};

const SEED: i64 = 109832;

#[derive(Debug, Clone)]
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
	let content = fs::read(path).context("Failed to read file")?;
	Ok(compute_checksum(&content))
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
	let mut state_bytes = Vec::new();

	for child in children {
		let child_state = get_state(child)?;
		state_bytes.extend_from_slice(&child_state.to_be_bytes());
	}

	Ok(compute_checksum(&state_bytes))
}

fn get_item(path: &str, items: &mut HashMap<String, Item>) -> Result<Item> {
	if let Some(item) = items.get(path).cloned() {
		return Ok(item);
	}

	let metadata = fs::metadata(path)
		.with_context(|| format!("Failed to read metadata for path: {}", path))?;
	let is_file = metadata.is_file();
	let is_folder = metadata.is_dir();
	let children = if is_folder { Some(get_children(path, items)?) } else { None };

	let mut item = Item {
		path: path.to_string(),
		is_file,
		is_folder,
		children,
		state: None,
	};

	item.state = Some(get_state(&item)?);
	items.insert(path.to_string(), item.clone());

	Ok(item)
}

fn get_children(path: &str, items: &mut HashMap<String, Item>) -> Result<Vec<Item>> {
	fs::read_dir(path)
		.with_context(|| format!("Failed to read directory: {}", path))?
		.map(|entry| {
			let entry = entry.with_context(|| format!("Failed to read entry in directory: {}", path))?;
			let path_str = entry.path().to_string_lossy().to_string();
			get_item(&path_str, items).with_context(|| format!("Failed to get item for path: {}", path_str))
		})
		.collect()
}

fn remove_duplicate_folders(items: &HashMap<String, Item>) -> Result<()> {
	let mut seen_states = HashSet::new();

	for (path, item) in items.iter() {
		if let Some(state) = &item.state {
			if seen_states.contains(state) && item.is_folder {
				println!("Removing: {}", path);
				// Handle the directory removal with proper error handling
				// fs::remove_dir_all(path)
				// 	.with_context(|| format!("Failed to remove directory: {}", path))?;
			}
			else {
				seen_states.insert(state.clone());
			}
		}
	}

	Ok(())
}

fn main() -> Result<()> {
	let root = env::current_dir().context("Failed to get current directory")?;
	let mut items = HashMap::new();
	let root_item = get_item(root.to_str().ok_or_else(|| anyhow::anyhow!("Path is not valid UTF-8"))?, &mut items)?;
	remove_duplicate_folders(&items)?;
	Ok(())
}
