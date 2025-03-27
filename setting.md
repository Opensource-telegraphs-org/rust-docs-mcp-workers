# Rust Docs MCP - Windsurf 集成指南

## 1. 调整 curl 命令

Windsurf 会通过 stdin 提供类似 `{"command": "lookup_crate_docs", "args": {"crate_name": "tokio"}}` 的 JSON 数据。我们需要让 curl：

- 使用 `@-` 从 stdin 读取 POST 数据
- 输出响应体到 stdout

### 调整后的 curl 命令：

```bash
curl -X POST http://localhost:7777/mcp -H "Content-Type: application/json" -d @- -s
```

参数说明：
- `-d @-`：从标准输入读取请求体
- `-s`：静默模式，避免输出进度信息

默认情况下，curl 会将响应体输出到 stdout，正好符合 Windsurf 的期望。

## 2. 配置 mcp_config.json

将 curl 命令写入 Windsurf 的 MCP 配置文件（通常位于 `~/.codeium/windsurf/mcp_config.json`）：

```json
{
  "mcpServers": {
    "local-mcp": {
      "command": "curl",
      "args": [
        "-X", "POST",
        "http://localhost:7777/mcp",
        "-H", "Content-Type: application/json",
        "-d", "@-",
        "-s"
      ]
    }
  }
}
```

参数说明：
- `"command": "curl"`：指定使用 curl 可执行文件
- `"args"`：将 curl 的参数拆分为数组形式，每个参数单独列出
  - `-X POST`：指定 POST 请求
  - `http://localhost:7777/mcp`：目标 URL
  - `-H Content-Type: application/json`：设置 JSON 头
  - `-d @-`：从 stdin 读取数据
  - `-s`：静默模式

## 3. 测试配置

### 确保服务运行
确认你的 MCP 服务在 http://localhost:7777/mcp 上运行，并且能正确响应 POST 请求。

### 手动测试 curl
在终端运行以下命令，验证 curl 是否能正确工作：

```bash
echo '{"command": "lookup_crate_docs", "args": {"crate_name": "tokio"}}' | curl -X POST http://localhost:7777/mcp -H "Content-Type: application/json" -d @- -s
```

如果返回的是 MCP 服务的响应（例如 tokio 的文档数据），说明命令有效。

### 刷新 Windsurf 配置
在 Windsurf 设置（Windsurf Settings > Cascade > Model Context Protocol (MCP) Servers）中点击"Refresh"，加载新的 MCP 配置。

### 在 Cascade 中测试
在 Cascade 聊天界面输入：

```
使用 mcp 执行 lookup_crate_docs，参数 crate_name 为 tokio
```

Windsurf 会将请求通过 curl 发送到你的服务，并显示响应。
