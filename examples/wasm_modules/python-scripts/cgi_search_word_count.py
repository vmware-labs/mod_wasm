#!/usr/bin/python3

import os, sys

print("Content-Type: text/plain")
print("")

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
	print("ERROR! Set FILE and WORD headers!")
	print_available_files()
	sys.exit()

try:
	file = open(FILE, "r")
except Exception as e:
	print(f"ERROR! Couldn't open file {FILE}! {e}", file=sys.stderr)
	print(f"ERROR! Couldn't open file {FILE}! {e}")
	print_available_files()

read_data = file.read()
word_count = read_data.count(WORD)

print(f"The word '{WORD}' appeared {word_count} times.")
