既然你已经重构了底层架构并去除了冗余的全局函数，README 也需要随之“进化”，重点突出**对象式调用**带来的丝滑体验。

以下是为你优化后的 **README.md**。我修正了版本号，精简了 API 说明，并加入了你发现的那个“模块化管理”的最佳实践建议。

---

# 🚀 QEnv (v0.2.0)

一个极致轻量、类型安全、零克隆（Zero-Clone）的 Rust 环境变量管理框架。

[![Crates.io](https://img.shields.io/crates/v/qenv.svg)](https://crates.io/crates/qenv) [![Documentation](https://docs.rs/qenv/badge.svg)](https://docs.rs/qenv) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YooRarely/qenv-rs)

## ✨ v0.2.0 核心进化

* 🪄 **无需 Trait 导入**: 通过 `define!` 宏生成的常量可以直接调用 `.get()` 或 `.take()`。
* 🛡️ **强类型契约**: 移除了不可控的字符串接口，强制通过 Tag 访问，从根源消除拼写错误。
* 🧩 **模块化友好**: 支持在独立模块（如 `env.rs`）中批量定义，全项目路径访问。

## 📦 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qenv = "0.2.0"
```

## 🛠️ 快速开始

### 1. 定义与初始化

建议在一个集中的位置（如 `mod env`）定义你的环境变量：

```rust
// src/env.rs
qenv::define! {
    PORT: "8080",                               // 带默认值
    DATABASE_URL: "postgres://localhost:5432",  // 必填项（若 .env/系统环境无此值则报错）
    IS_DEBUG: "false"                           // 自动类型转换支持
}
```

### 2. 初始化与使用

```rust
use qenv;
mod env; // 引入你定义的变量

fn main() {
    // 1. 初始化（自动加载 .env 并缓存到内存）
    qenv::init().expect("QEnv 初始化失败");

    // 2. 极致丝滑的访问体验 (零克隆获取 &'static str)
    let db_url = env::DATABASE_URL.get();
    
    // 3. 自动类型转换 (.take())
    let port: u16 = env::PORT.take();
    let is_debug: bool = env::IS_DEBUG.take();

    println!("Listening on {}, debug: {}", port, is_debug);
}
```

### 3. 安全处理 (Try 系列)

如果不确定变量是否存在，或者需要处理转换错误，可以使用 `try_` 方法：

```rust
// 不带默认值的变量
qenv::define!(OPTIONAL_CONFIG);

match OPTIONAL_CONFIG.try_get() {
    Ok(val) => println!("Value: {}", val),
    Err(e) => eprintln!("Handle error: {}", e),
}

// 尝试转换类型
if let Ok(val) = env::PORT.try_take::<u32>() {
    // ...
}
```

## 🧩 进阶配置

### 性能说明

QEnv 在初始化时将所有环境变量一次性存入 `OnceLock<HashMap>`。后续所有的 `.get()` 操作均返回 **字符串引用**。
这意味着在你的业务逻辑中频繁读取环境变量是**零开销**的，完全不需要 `clone()`。

### 关闭默认的 Dotenv 支持

在 Docker 环境或生产环境中，你可能不需要 `.env` 文件支持：

```toml
[dependencies]
qenv = { version = "0.2.0", default-features = false }
```

## 📑 错误类型说明

`qenv` 提供结构化的 `EnvError` 以便精确处理异常：

* `InitializeError`: 重复调用 `init()`。
* `NotInitialized`: 未调用 `init()` 就尝试读取。
* `Missing(name)`: 变量缺失且没有默认值。
* `ParseError`: 字符串转换为目标类型（如 `u16`, `f64`, `bool`）失败。

## 🤝 贡献

欢迎提交 Issue 或 Pull Request！

## 📜 许可证

本项目采用 [MIT](https://www.google.com/search?q=https://github.com/YooRarely/qenv-rs/blob/main/LICENSE) 许可证。

---
