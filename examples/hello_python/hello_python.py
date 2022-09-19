import os

print("Content-Type: text/plain;")
print("")

# Env Vars
print("*** ENV VARS ***")
for k, v in sorted(os.environ.items()):
    print(k+':', v)