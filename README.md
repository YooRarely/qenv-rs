
---

# 🚀 QEnv (v0.3.0)

一个极致轻量、类型安全、零克隆（Zero-Clone）且具备**启动校验**功能的 Rust 环境变量管理框架。

[![Crates.io](https://img.shields.io/crates/v/qenv.svg)](https://crates.io/crates/qenv) [![Documentation](https://docs.rs/qenv/badge.svg)](https://docs.rs/qenv) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YooRarely/qenv-rs)

## ✨ v0.3.0 核心魔法

* 🛡️ **启动即校验 (Fail-Fast)**: 宏自动生成的 `init()` 会检查对应缺失配置，确保程序不会在运行时因缺少环境变量而崩溃。
* 🪄 **自动解引用 (Deref & AsRef)**: 代理对象可直接作为 `&str` 使用，完美兼容 `tracing`、`std::fs` 等标准库接口。
* 📺 **原生显示 (Display)**: 支持在 `format!` 或 `println!` 中直接占位，无需 `.get()`。
* ⚡ **零克隆 (Zero-Clone)**: 基于静态生命周期引用，读取性能等同于直接访问常量。

## 📦 安装

```toml
[dependencies]
qenv = "0.3.0"
```

## 🛠️ 快速开始

### 1. 定义环境变量

建议在独立模块（如 `src/env.rs`）中使用宏定义，这会自动在该模块下生成 `init()` 函数：

```rust
// src/env.rs
use qenv;
qenv::define! {
    PORT: "8080",                               // 默认值
    DATABASE_URL: "postgres://localhost:5432",  // 必填项（若无默认值且环境缺失则 init 报错）
    RUST_LOG: "info"
}
```

### 2. 初始化与使用

```rust

mod env;

fn main() {
    // ✨ 核心进化：调用模块级 init()，自动加载 .env 并校验所有定义的变量
    env::init().expect("QEnv 校验失败：缺少必要的环境变量");

    // ✨ 魔法 1: 直接作为 &str 传参 (兼容 AsRef/Deref)
    tracing_subscriber::fmt()
        .with_env_filter(&env::RUST_LOG) 
        .init();

    // ✨ 魔法 2: 直接在 format! 中使用 (Display 实现)
    let addr = format!("0.0.0.0:{}", env::PORT);
    
    // 🎯 强类型转换
    let port: u16 = env::PORT.take();

    println!("🚀 Server running at http://{}", addr);
}
```

## 🧩 进阶特性

### 模块化管理

你可以根据功能定义多个环境模块，并分别初始化：

```rust
mod db_env {
	use qenv;
	qenv::define! { DB_URL } 
}
mod s3_env { 
	use qenv;
	qenv::define! { S3_KEY, S3_REGION: "us-east-1" } 
}

fn main() {
    db_env::init().unwrap(); // 仅校验数据库相关
    s3_env::init().unwrap(); // 仅校验 S3 相关
}
```

### 零开销抽象

QEnv 内部使用 **ZST (Zero Sized Types)**。定义的每个变量在内存中都不占用空间，所有的 `.get()`、`.take()` 方法调用都会被编译器内联优化。

## 📑 错误处理说明

`qenv` 提供结构化的 `EnvError` 以便精确处理异常：

* `InitializeError`: 全局缓存重复初始化。
* `NotInitialized`: 未执行 `init()` 导致的读取失败。
* `Missing(name)`: 变量缺失且没有提供默认值。
* `ParseError`: 字符串转换为目标类型（如 `u16`, `bool`）失败。
---

## 🤝 贡献

欢迎提交 Issue 或 Pull Request！

## 📜 许可证

本项目采用 [MIT](https://www.google.com/search?q=https://github.com/YooRarely/qenv-rs/blob/main/LICENSE) 许可证。

---
