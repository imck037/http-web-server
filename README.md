# Rust HTTP Web Server

A lightweight and fast HTTP web server built in **Rust**, designed for performance, safety, and simplicity.

## Features

* High performance and low memory usage
* Memory-safe (thanks to Rust)
* Handles HTTP requests and responses
* Concurrent request handling 
* Minimal dependencies 
* Easy to extend and customize

## Project Structure

```
.
├── src/
│   ├── main.rs        # Entry point
│   ├── lib.rs         # Server logic
├── Cargo.toml         # Dependencies and metadata
└── README.md
```

## Installation

Make sure you have Rust installed:

```bash
rustc --version
cargo --version
```

If not, install from: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Clone the repository:

```bash
git clone https://github.com/imck037/http-web-server.git
cd http-web-server
```

## Running the Server

```bash
cargo run
```

By default, the server runs on:

```
http://127.0.0.1:2020
```

## Example Request

Using curl:

```bash
curl http://127.0.0.1:2020/
```

Example response:

```
<!DOCTYPE html>
<html lang="en">

<head>
	<meta charset="utf-8">
	<title>Hello!</title>
</head>

<body>
	<h1>Hello!</h1>
	<p>Hi from Rust</p>
</body>

</html>
```

## How It Works

1. The server binds to a TCP socket
2. Listens for incoming connections
3. Parses HTTP requests
4. Routes requests to handlers
5. Sends back HTTP responses
