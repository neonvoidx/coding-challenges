import argparse
import http.server
import multiprocessing
import signal
import socketserver
import sys

BASE_PORT = 8000


def serve(port):
    # Ignore SIGINT in child — parent handles shutdown and terminates us explicitly
    signal.signal(signal.SIGINT, signal.SIG_IGN)
    Handler = http.server.SimpleHTTPRequestHandler
    Handler.log_message = lambda *args: None  # suppress per-request logs
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(("", port), Handler) as httpd:
        print(f"Serving at http://127.0.0.1:{port}", flush=True)
        httpd.serve_forever()


def main():
    parser = argparse.ArgumentParser(
        description="Spin up N HTTP servers serving index.html"
    )
    parser.add_argument("count", type=int, help="Number of HTTP servers to start")
    args = parser.parse_args()

    processes = []
    for i in range(args.count):
        port = BASE_PORT + i
        p = multiprocessing.Process(target=serve, args=(port,), daemon=True)
        p.start()
        processes.append(p)

    def shutdown(sig, frame):
        print("\nShutting down servers...")
        for p in processes:
            p.terminate()
        for p in processes:
            p.join(timeout=3)
            if p.is_alive():
                p.kill()
        sys.exit(0)

    signal.signal(signal.SIGINT, shutdown)
    signal.signal(signal.SIGTERM, shutdown)

    for p in processes:
        p.join()


if __name__ == "__main__":
    main()
