# Updraft Solana 学习笔记

这是一个用于学习 Solana 区块链开发的系统性练习集。本项目旨在通过实践，帮助开发者从基础的 Native 开发逐步过度到使用 Anchor 框架进行高效开发。

## 项目结构

本项目目前包含两个主要的练习目录：

- `hello-native/`: **Solana 原生开发练习**。使用 Solana SDK 直接进行程序开发。
  - `src/lib.rs`: 链上程序逻辑。
  - `examples/client.rs`: 使用 Rust 编写的客户端 RPC 调用示例。
- `hello-anchor/`: **Anchor 框架开发练习**。使用更现代、安全的 Anchor 框架进行开发。
  - `programs/hello-anchor/`: Anchor 程序源代码。
  - `tests/`: 使用 TypeScript 或 Rust 编写的集成测试。

## 开发环境要求

- [Rust](https://www.rust-lang.org/tools/install) (最新稳定版)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (建议 v1.18+)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation) (建议 v0.30+)
- [Node.js](https://nodejs.org/) & [pnpm](https://pnpm.io/) (用于 Anchor 测试)

## 快速开始

### 1. Solana 原生程序 (hello-native)

#### 编译与测试

```bash
cd hello-native
cargo build-sbf
cargo test -- --nocapture
```

#### 部署与运行 (本地测试网)

1. 启动验证节点: `solana-test-validator`
2. 部署程序: `solana program deploy target/deploy/hello_world.so`
3. 运行客户端: `cargo run --example client`

---

### 2. Anchor 程序 (hello-anchor)

#### 安装依赖

```bash
cd hello-anchor
pnpm install
```

#### 编译与测试

```bash
anchor build
anchor test
```

## 学习路线图

1. [x] **Phase 1**: Hello World (Native & Anchor)
2. [ ] **Phase 2**: 账户模型与状态管理 (Account & State)
3. [ ] **Phase 3**: 程序派生地址 (PDA)
4. [ ] **Phase 4**: Token 程序与 SPL 交互
5. [ ] **Phase 5**: 跨程序调用 (CPI)
6. [ ] **Phase 6**: 去中心化应用 (DApp) 实战

## 许可证

MIT
