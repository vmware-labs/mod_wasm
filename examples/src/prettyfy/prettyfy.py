#!/usr/bin/python3

import os, sys, cgi 
from pygments import highlight
from pygments.lexers import get_lexer_by_name
from pygments.styles import get_style_by_name
from pygments.formatters import HtmlFormatter

# Create instance of FieldStorage 
form = cgi.FieldStorage() 

print("Content-Type: text/html")
print("")

print("<!DOCTYPE html><HTML><HEAD>")
print("<TITLE>PrettyFy</TITLE><meta charset=\"utf-8\">")
print("<style>body{font-family:-apple-system,BlinkMacSystemFont,avenir next,avenir,segoe ui,helvetica neue,helvetica,Cantarell,Ubuntu,roboto,noto,arial,sans-serif;background-color:#fff;margin:0}main{margin-bottom:0.1rem}header{padding:0.1rem;background:linear-gradient(60deg,#d1ebff,#99c2ff);margin-bottom:0.1rem}.content{max-width:1300px;margin:0 auto;padding:0 1rem}h1{text-align:center}h2{border-bottom:1px solid #aaa;padding-bottom:.5rem}pre{padding:0.5rem;border:1px solid #ccc;font-size:.9rem;border-radius:5px;background-color:#f6f6f6;font-family:Menlo,Consolas,Monaco,Liberation Mono,Lucida Console,monospace;white-space:pre-wrap}.var{font-weight:700}</style>")
print("</HEAD><BODY><main>")
print('<header><h1>💻 PrettyFy 🎨</h1></header>')
print('<div class="content">')

# Platform
print('<h2>Platform</h2>')
print('<pre><code>', end='')
print('<span class="var">sys.platform = ' + '</span>'+ sys.platform)
print('</code></pre>')

# Get files at "uploads/"
path = "uploads/"
if path:
    print('<h2>Available files at \'' + path + '\'</h2>')
    print('<pre><code>', end='')
    try:
        dirs = os.listdir(path)
        for entry in dirs:
            print('<span class="var">'+ entry + '</span>')
    except Exception as e:
        print("ERROR! " + str(e))
    print('</code></pre>')

file = form.getvalue("file")
if file:
    filepath=path+file
    print('<h2>Try opening file: \'' + filepath + '\'</h2>')
    try:
        file = open(filepath, 'r')
    except Exception as e:
        print("ERROR! " + str(e))
    if file:
        code = file.read()
        lexer = get_lexer_by_name("python", stripall=True)
        style = get_style_by_name('colorful')
        formatter = HtmlFormatter(style='colorful')
        result = highlight(code, lexer, formatter)
        style = formatter.get_style_defs()
        print("<style>" + style + "</style>")
        print(result)

print("</div></main></BODY></HTML>")
