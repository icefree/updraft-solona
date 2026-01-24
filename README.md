# Updraft Solana 学习笔记

这是一个用于学习 Solana 区块链开发的练习集。本项目将包含多个阶段的练习，从基础的入门合约到复杂的去中心化应用（DApp）。

## 已包含的练习

- `hello_world/`: **入门练习**。包含一个简单的 "Hello World" 合约以及一个 Rust 编写的客户端调用示例。
  - `src/lib.rs`: 链上程序逻辑。
  - `examples/client.rs`: 客户端 RPC 调用示例。

> [!TIP]
> 这是一个持续更新的项目，未来将加入更多关于账户状态、Token 程序、PDA 以及 Anchor 框架的进阶练习。

- `README.md`: 项目说明文件。

## 开发环境要求

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)

## 快速开始

### 1. 编译 Solana 程序

```bash
cd hello_world
cargo build-sbf
cargo test -- --nocapture
```

### 2. 部署程序 (本地测试网)

首先启动本地验证节点：

```bash
solana-test-validator
```

在另一个终端中部署：

```bash
solana program deploy target/deploy/hello_world.so
```

### 3. 运行客户端示例

确保本地测试网正在运行，且程序已成功部署。

```bash
cd hello_world
cargo run --example client
```

> [!NOTE]
> 运行客户端前，请确保 `hello_world/examples/client.rs` 中的 `program_id` 与你部署后的程序 ID 一致。

## 许可证

MIT
