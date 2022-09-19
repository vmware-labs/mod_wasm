from datetime import datetime
import sys, os

print("<!DOCTYPE html><HTML><HEAD>")
print("<TITLE>hello python!</TITLE><meta charset=\"utf-8\">");
print("<style>body{font-family:-apple-system,BlinkMacSystemFont,avenir next,avenir,segoe ui,helvetica neue,helvetica,Cantarell,Ubuntu,roboto,noto,arial,sans-serif;background-color:#fff;margin:0}main{margin-bottom:0.1rem}header{padding:0.1rem;background:linear-gradient(60deg,#d1ebff,#99c2ff);margin-bottom:0.1rem}.content{max-width:1300px;margin:0 auto;padding:0 1rem}h1{text-align:center}h2{border-bottom:1px solid #aaa;padding-bottom:.5rem}pre{padding:0.5rem;border:1px solid #ccc;font-size:.9rem;border-radius:5px;background-color:#f6f6f6;font-family:Menlo,Consolas,Monaco,Liberation Mono,Lucida Console,monospace;white-space:pre-wrap}.var{font-weight:700}</style>")
print("</HEAD><BODY><main>")
print('<header><h1>Hello from Python! 👋</h1></header>')
print('<div class="content">')

# Info
print('<h2>Info</h2>')
print('<pre><code>', end='')
print("datetime.now():", datetime.now().strftime("%d/%m/%Y %H:%M:%S"))
print('sys.platform:', sys.platform)
print('sys.version_info:', sys.version_info)
print('</code></pre>')

# Args
print('<h2>Arguments</h2>')
print('<pre><code>sys.argv:', sys.argv, '</code></pre>')

# Env Vars
print('<h2>Environment Variables</h2>')
print('<pre><code>', end='')
for k, v in sorted(os.environ.items()):
    print('<span class="var">'+ k + '=' + '</span>'+ v)
print('</code></pre>')

print("</div></main></BODY></HTML>")