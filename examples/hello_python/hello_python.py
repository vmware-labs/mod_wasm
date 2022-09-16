from datetime import datetime
import sys
import os

print("Content-Type: text/plain")
print("")

# Datetime and Sys
print("*** INFO ***")
now = datetime.now()
dt_string = now.strftime("%d/%m/%Y %H:%M:%S")
print("Date and time:", dt_string)
print("sys.platform:", sys.platform)
print()

# Env Vars
print("*** ENV VARS ***")
for k, v in sorted(os.environ.items()):
    print(k+':', v)

