# 项目概述

本项目是一个包含多个子项目的 Rust 应用程序示例。项目包括一个 HTTP 库、一个 HTTP 服务器、一个 TCP 客户端和一个 TCP
服务器。每个子项目都有自己的功能模块和配置。

## 子项目说明

### `http`

这是一个用于处理 HTTP 请求和响应的库。包含以下文件：

- `httprequest.rs`: 定义 HTTP 请求的结构和功能。
- `httpresponse.rs`: 定义 HTTP 响应的结构和功能。
- `lib.rs`: 库的入口文件，提供公共 API。

### `httpserver`

这是一个 HTTP 服务器实现，能够处理请求和发送响应。包含以下文件：

- `handler.rs`: 处理 HTTP 请求的逻辑。
- `main.rs`: 服务器的入口点。
- `router.rs`: 路由配置和请求分发。
- `server.rs`: 服务器的核心功能。
- `data/orders.json`: 示例数据文件。
- `public/`: 存放静态文件（如 HTML 和 CSS）。

### `tcpclient`

这是一个简单的 TCP 客户端实现。包含以下文件：

- `main.rs`: 客户端的入口点。

### `tcpserver`

这是一个简单的 TCP 服务器实现。包含以下文件：

- `main.rs`: 服务器的入口点。

## 依赖

请确保已安装 Rust 环境，并运行以下命令来获取依赖：

```bash
cargo build
```

## 使用说明

1. **编译项目**：
   ```bash
   cargo build
   ```

2. **运行 HTTP 服务器**：
   ```bash
   cd httpserver
   cargo run
   ```

3. **运行 TCP 客户端**：
   ```bash
   cd tcpclient
   cargo run
   ```

4. **运行 TCP 服务器**：
   ```bash
   cd tcpserver
   cargo run
   ```

## 参考资料

- [Rust Servers, Services, and Apps](books/rust-servers-services-and-apps.pdf): 本书提供了有关 Rust 服务器、服务和应用程序的深入知识。

## 贡献

如果您有任何建议或改进，请提交 Pull Request 或报告问题。

## 许可证

本项目采用 [MIT 许可证](LICENSE)。