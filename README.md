
---

# 🚀 QEnv (v0.2.1)

一个极致轻量、类型安全、零克隆（Zero-Clone）的 Rust 环境变量管理框架。

[![Crates.io](https://img.shields.io/crates/v/qenv.svg)](https://crates.io/crates/qenv) [![Documentation](https://docs.rs/qenv/badge.svg)](https://docs.rs/qenv) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/YooRarely/qenv-rs)

## ✨ v0.2.2 核心魔法

* 🪄 **自动解引用 (Deref)**: 代理对象可直接作为 `&str` 使用，无需调用 `.get()`。
* 📺 **原生显示 (Display)**: 支持在 `format!` 或 `println!` 中直接占位。
* 🛡️ **强类型契约**: 彻底摒弃字符串硬编码，通过宏生成 ZST Tag，编译期消除拼写隐患。
* ⚡ **零克隆 (Zero-Clone)**: 核心基于 `str` 引用，读取性能与直接访问常量无异。

## 📦 安装

```toml
[dependencies]
qenv = "0.2.2"

```

## 🛠️ 快速开始

### 1. 定义环境变量

建议在 `src/env.rs` 中统一管理：

```rust
qenv::define! {
    PORT: "8080",                               // 默认值
    DATABASE_URL: "postgres://localhost:5432",  // 环境变量覆盖
    RUST_LOG: "info"
}

```

### 2. 极致丝滑的使用体验

```rust
use qenv;
mod env;

fn main() {
    qenv::init().expect("QEnv 初始化失败");

    // ✨ 魔法 1: 直接作为 &str 传参 (Deref 强制转换)
    // 无需 .get()，编译器会自动处理
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

## 🧩 高级特性

### 安全处理 (Try 系列)

当你需要精细控制变量缺失或解析失败的情况：

```rust
match env::DATABASE_URL.try_get() {
    Ok(url) => connect(url),
    Err(e) => panic!("配置错误: {}", e),
}

if let Ok(debug) = env::IS_DEBUG.try_take::<bool>() {
    // 执行调试逻辑
}

```

### 零开销抽象

QEnv 内部使用结构体代理模式。定义的每个变量在内存中都是 **0 字节** (ZST)，方法调用在编译后会被内联。这意味着你获得的不仅仅是语法糖，还有不打折扣的性能。

### 可选特性

| Feature | 描述 | 默认开启 |
| --- | --- | --- |
| `dotenv` | 支持自动加载 `.env` 文件 | 是 |

若在生产环境（如 K8s/Docker）不需要加载 `.env`，可禁用以减少依赖：

```toml
qenv = { version = "0.2.2", default-features = false }

```

## 📑 错误处理

`qenv` 提供清晰的错误分类：

* `InitializeError`: 重复初始化。
* `NotInitialized`: 未执行 `init()`。
* `Missing(name)`: 缺失且无默认值。
* `ParseError`: 类型转换失败。

---

## 🤝 贡献

欢迎提交 Issue 或 Pull Request！

## 📜 许可证

本项目采用 [MIT](https://www.google.com/search?q=https://github.com/YooRarely/qenv-rs/blob/main/LICENSE) 许可证。

---
