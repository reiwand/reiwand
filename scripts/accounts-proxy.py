import http.client, http.server, os, ssl, sys

port, backend, cert, key = int(sys.argv[1]), sys.argv[2], sys.argv[3], sys.argv[4]
record = sys.argv[5] if len(sys.argv) > 5 else ""


class Front(http.server.BaseHTTPRequestHandler):
    protocol_version = "HTTP/1.1"

    def log_message(self, *args):
        pass

    def reply(self, status, headers, data):
        self.send_response(status)
        for name, value in headers:
            if name.lower() not in ("connection", "transfer-encoding", "content-length", "keep-alive"):
                self.send_header(name, value)
        self.send_header("content-length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def handle_any(self):
        size = int(self.headers.get("content-length") or 0)
        body = self.rfile.read(size) if size else None
        if record:
            with open(record, "a") as out:
                out.write(self.requestline + "\n")
                out.writelines(f"{k.lower()}: {v}\n" for k, v in self.headers.items())
        if backend.startswith("dir:"):
            path = os.path.join(backend[4:], self.path.split("?")[0].lstrip("/"))
            if os.path.isdir(path):
                path = os.path.join(path, "index.html")
            if os.path.isfile(path):
                return self.reply(200, [], open(path, "rb").read())
            return self.reply(404, [], b"")
        host, number = backend.split(":")
        link = http.client.HTTPConnection(host, int(number), timeout=60)
        skip = ("connection", "keep-alive", "transfer-encoding")
        link.request(self.command, self.path, body, {k: v for k, v in self.headers.items() if k.lower() not in skip})
        answer = link.getresponse()
        self.reply(answer.status, answer.getheaders(), answer.read())

    do_GET = do_POST = do_PUT = do_PATCH = do_DELETE = do_HEAD = handle_any


server = http.server.ThreadingHTTPServer(("127.0.0.1", port), Front)
context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
context.load_cert_chain(cert, key)
server.socket = context.wrap_socket(server.socket, server_side=True)
server.serve_forever()
