import os, sys

FILE = os.getenv('HTTP_FILE')
WORD = os.getenv('HTTP_WORD')

if FILE == None or WORD == None:
	print("ERROR! Set FILE and WORD headers!", file=sys.stderr)
	sys.exit()

try:
	file = open(FILE, "r")
except Exception as e:
	print(f"ERROR! Couldn't open file {FILE}! {e}", file=sys.stderr)

read_data = file.read()
word_count = read_data.count(WORD)

print("Content-Type: text/plain")
print("")
print(f"The word '{WORD}' appeared {word_count} times.")
