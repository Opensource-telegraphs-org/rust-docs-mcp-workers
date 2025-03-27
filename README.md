# Rust Docs MCP Server

A web server that provides Rust crate documentation through an HTTP API endpoint. This service allows you to fetch and search documentation for Rust crates programmatically.

## Features

- HTTP endpoint for fetching Rust crate documentation
- JSON-based API for easy integration
- Support for querying any published crate on crates.io
- Automatic parsing and formatting of documentation
- Configurable via environment variables

## Prerequisites

- Rust and Cargo (latest stable version recommended)
- Internet connection (to fetch crate documentation)

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/rust-docs-mcp-workers.git
   cd rust-docs-mcp-workers
   ```

2. Create an `.env` file from the example:
   ```bash
   cp .env.example .env
   ```

3. Edit the `.env` file to configure your settings:
   ```
   PORT=7777  # The port on which the server will run
   ```

4. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Starting the Server

```bash
# Run in development mode
cargo run

# Run with specific logging level
RUST_LOG=info cargo run

# Run the compiled binary directly
./target/release/rust-docs-mcp-server
```

The server will start and listen for HTTP requests on the configured port (default: 6666 or from the PORT environment variable).

### API Endpoints

#### POST /mcp

Fetch documentation for a specific Rust crate.

**Request Format:**

```json
{
  "command": "lookup_crate_docs",
  "args": {
    "crate_name": "tokio"
  }
}
```

**Parameters:**

- `command`: Must be `"lookup_crate_docs"` (required)
- `args.crate_name`: The name of the crate to look up (optional, defaults to "tokio")

**Response Format:**

```json
{
  "content": [
    {
      "type_": "text",
      "text": "# Tokio\n\nA runtime for writing reliable network applications without compromising speed.\n\n..."
    }
  ],
  "is_error": null
}
```

**Error Response:**

```json
{
  "content": [
    {
      "type_": "text",
      "text": "Error message"
    }
  ],
  "is_error": true
}
```

### Example Request with curl

```bash
curl -X POST http://localhost:7777/mcp \
  -H "Content-Type: application/json" \
  -d '{"command": "lookup_crate_docs", "args": {"crate_name": "serde"}}'
```

## HTTP API 访问指南

服务器启动后，可以通过 HTTP 协议访问文档服务。服务默认监听在 `http://localhost:7777` (或环境变量 `PORT` 指定的端口)。

### API 端点

#### POST /mcp

查询 Rust crate 文档的主要端点。

**请求格式:**

```json
{
  "command": "lookup_crate_docs",
  "args": {
    "crate_name": "目标crate名称"
  }
}
```

**参数说明:**

- `command`: 必须为 `"lookup_crate_docs"`
- `args.crate_name`: 要查询的 crate 名称，可选，默认为 "tokio"

**响应格式:**

```json
{
  "content": [
    {
      "type_": "text",
      "text": "返回的文档内容..."
    }
  ],
  "is_error": null
}
```

### 多种客户端访问示例

#### 使用 curl

```bash
# 查询 tokio crate 的文档
curl -X POST http://localhost:7777/mcp \
  -H "Content-Type: application/json" \
  -d '{"command": "lookup_crate_docs"}'

# 查询特定 crate (例如 serde) 的文档
curl -X POST http://localhost:7777/mcp \
  -H "Content-Type: application/json" \
  -d '{"command": "lookup_crate_docs", "args": {"crate_name": "serde"}}'
```

#### 使用 Python

```python
import requests
import json

url = "http://localhost:7777/mcp"
headers = {"Content-Type": "application/json"}

# 查询 actix-web crate 的文档
payload = {
    "command": "lookup_crate_docs",
    "args": {
        "crate_name": "actix-web"
    }
}

response = requests.post(url, headers=headers, data=json.dumps(payload))
print(response.json())
```

#### 使用 JavaScript (Node.js)

```javascript
const fetch = require('node-fetch');

const url = 'http://localhost:7777/mcp';
const headers = { 'Content-Type': 'application/json' };

// 查询 tokio crate 的文档
const payload = {
    command: 'lookup_crate_docs',
    args: {
        crate_name: 'tokio'
    }
};

fetch(url, {
    method: 'POST',
    headers: headers,
    body: JSON.stringify(payload)
})
.then(response => response.json())
.then(data => console.log(data))
.catch(error => console.error('Error:', error));
```

#### 使用 Rust

```rust
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    
    // 查询 serde crate 的文档
    let response = client.post("http://localhost:7777/mcp")
        .header("Content-Type", "application/json")
        .json(&json!({
            "command": "lookup_crate_docs",
            "args": {
                "crate_name": "serde"
            }
        }))
        .send()
        .await?;
    
    let result: Value = response.json().await?;
    println!("{:#?}", result);
    
    Ok(())
}
```

### 服务状态确认

可以使用简单的 HTTP 请求来确认服务是否正常运行：

```bash
# 使用 curl 确认服务状态
curl -X POST http://localhost:7777/mcp \
  -H "Content-Type: application/json" \
  -d '{"command": "lookup_crate_docs", "args": {"crate_name": "tokio"}}' \
  -o /dev/null -s -w "%{http_code}\n"
```

如果服务正常运行，应该返回 `200` 状态码。

## in Windsurf ide

Windsurf is a powerful agent AI IDE that supports a variety of languages including rust. To integrate your rust-docs-mcp-server with Windsurf, follow these steps:

### Using rust-docs-mcp-server in Windsurf


### 测试配置
在 Cascade 面板输入：

Use the lookup_crate_docs tool to get documentation for "tokio".

Windsurf 将发送 POST 请求，返回文档内容。


## Project Structure

- `src/main.rs` - Main application code
- `Cargo.toml` - Project dependencies and configuration
- `.env` - Environment configuration (not committed to repository)
- `.env.example` - Example environment configuration

## Dependencies

- `actix-web` - HTTP server framework
- `serde` - Serialization/deserialization
- `reqwest` - HTTP client
- `scraper` - HTML parsing
- `tokio` - Asynchronous runtime
- `tracing` - Logging and diagnostics
- `dotenvy` - Environment variable loading

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust Documentation](https://docs.rs/)
- [Actix Web](https://actix.rs/)
- [Tokio](https://tokio.rs/)
