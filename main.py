import os
import re
import shutil
import subprocess

escape = lambda text: re.sub(r"(.)", r"\\\1", text)

items = {}

def get_string_checksum(input_string):
	try:
		result = subprocess.run(
			["xxhsum", "-"],
			input=input_string,
			capture_output=True,
			text=True,
			check=True
		)
		checksum = result.stdout.split()[0]
		return checksum
	except subprocess.CalledProcessError as e:
		print(f"Error computing checksum: {e}")
	except Exception as e:
		print(f"An error occurred: {e}")

def get_file_checksum(path):
	try:
		result = subprocess.run(["xxhsum", path], capture_output=True, text=True, check=True)
		checksum = result.stdout.split()[0]
		return checksum
	except subprocess.CalledProcessError as e:
		print(f"Error computing checksum: {e}")

def get_state(item):
	state = ""
	path = item["path"]
	if item["is_folder"]:
		for child in item["children"]:
			state = get_string_checksum(state + get_state(child))
	elif item["is_file"]:
		return get_file_checksum(path)
	return state

def get_checksum(item):
	path = item["path"]
	if item["is_file"]:
		return get_file_checksum(path)
	elif item["is_folder"]:
		return ""
	else:
		print("OOF")

def get_children(path):
	children = []
	for sub in os.listdir(path):
		duh = os.path.join(path, sub)
		children.append(get_item(duh))
	return children

def get_item(path):
	try:
		if items[path] != None:
			return items[path]
	except:
		item = {}
		item["path"] = path
		item["is_file"] = os.path.isfile(path)
		item["is_folder"] = os.path.isdir(path)
		if item["is_folder"]:
			item["children"] = get_children(path)
		item["state"] = get_state(item)
		items[path] = item
		return item

def remove_duplicate_folders():
	seen_states = set()
	# duplicates = set()

	for path in items:
		item = items[path]
		# print(item["path"])
		state = item["state"]
		if state in seen_states:
			if item["is_folder"]:
				print("removing:", item["path"])
				# shutil.rmtree(item["path"])
		else:
			seen_states.add(state)

def main():
	root = os.getcwd()
	root_item = get_item(root)
	remove_duplicate_folders()

if __name__ == "__main__":
	main()