#!/usr/bin/python3

import os, sys

print("Content-Type: text/plain", flush=True)
print("", flush=True)

def print_available_files():
	current_dir = "."
	print(f"Available files at '{current_dir}':")
	try:
		dirs = os.listdir(current_dir)
		for entry in dirs:
			print(f" - {entry}")
	except Exception as e:
		print("ERROR! " + str(e))

FILE = os.getenv('HTTP_FILE')
WORD = os.getenv('HTTP_WORD')

if FILE == None or WORD == None:
	print("ERROR! Set FILE and WORD headers!", file=sys.stderr)
	sample_call = 'curl -H "File: Sherlock.txt" -H "Word: elementary" http://localhost:8080/search-word-count'
	print(f"ERROR! Set FILE and WORD headers! For example like this:\n\n\t{sample_call}\n\n")
	print_available_files()
else:

	file = None
	try:
		file = open(FILE, "r")
	except Exception as e:
		print(f"ERROR! Couldn't open file {FILE}! {e}", file=sys.stderr)
		print(f"ERROR! Couldn't open file {FILE}! {e}")
		print_available_files()
	else:
		read_data = file.read()
		word_count = read_data.count(WORD)

		print(f"The word '{WORD}' appeared {word_count} times.")
