# 🚀 QEnv

一个极致轻量、类型安全、零克隆（Zero-Clone）的 Rust 环境变量管理库。

[![Crates.io](https://img.shields.io/crates/v/qenv.svg)](https://crates.io/crates/qenv)
[![Documentation](https://docs.rs/qenv/badge.svg)](https://docs.rs/qenv)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/your-user/qenv)

## ✨ 特性

- **⚡ 极致性能**: 基于 `'static str` 引用，读取环境变量无需克隆 `String`。
- **🛡️ 类型安全**: 通过宏定义 Tag 结构体，消除拼写错误，支持强类型转换。
- **📦 极简 API**: 只需 `use qenv;` 即可通过 `qenv::get(KEY)` 或 `KEY.get()` 自由切换。
- **🔌 灵活初始化**: 内置 `dotenvy` 支持（可选），支持全局单次初始化校验。

## 📦 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qenv = "0.1.0"


## 🛠️ 快速开始

### 1. 定义与初始化

```rust
use qenv;

// 使用宏批量定义环境变量 Tag
qenv::define! {
    PORT: "8080",                               // 带默认值
    DATABASE_URL: "postgres://localhost:5432",  // 也可以在 .env 中覆盖
    IS_DEBUG: "false"                           // 自动类型转换支持
}

fn main() {
    // 加载 .env 并初始化缓存
    qenv::init().expect("环境变量初始化失败");
}

```

### 2. 获取变量 (两种风格)

你可以根据喜好选择函数式调用或对象式调用：

```rust
// 风格 A: 函数式调用 (推荐，语义清晰)
let db = qenv::get(DATABASE_URL);
let port: u16 = qenv::take(PORT);

// 风格 B: 结构体方法调用
let debug = IS_DEBUG.take::<bool>();

```

### 3. 安全处理

如果不确定变量是否存在，可以使用 `try_` 系列方法：

```rust
match qenv::try_get(SOME_VAR) {
    Ok(val) => println!("Value: {}", val),
    Err(e) => eprintln!("Error: {}", e),
}

```

## 🧩 进阶配置

### 关闭默认的 Dotenv 支持

如果你不需要加载 `.env` 文件（例如在生产环境 Docker 中），可以关闭默认 Feature 以减少依赖：

```toml
[dependencies]
qenv = { version = "0.1.0", default-features = false }

```

## 📑 错误类型说明

`qenv` 提供结构化的 `EnvError` 以便精确处理异常：

* `InitializeError`: 重复初始化。
* `NotInitialized`: 未调用 `init()` 就尝试读取。
* `Missing(name)`: 变量缺失且没有默认值。
* `ParseError`: 字符串转换为目标类型（如 `i32`, `bool`）失败。

## 🤝 贡献

欢迎提交 Issue 或 Pull Request！

## 📜 许可证

本项目采用 [MIT](https://www.google.com/search?q=LICENSE-MIT) 许可证。



---
