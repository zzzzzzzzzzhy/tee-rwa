# TEE-RWA 隐私信用评估系统

基于 TEE 与 Solana 的供应链金融隐私信用评估系统。

## 架构亮点

- **RA-TLS 安全通道**: 密钥由硬件保护，后端全程只处理密文
- **SGX Sealing**: 解决 TEE 重启丢密钥问题
- **ECDSA 链上验证**: 轻量级验证，避免高 Gas 消耗
- **双向加密分发**: 平衡隐私保护与业务需求

## 快速开始

### 前置要求

- Docker & Docker Compose
- Rust 1.75+
- Node.js 18+
- Solana CLI (已配置 devnet)

### 启动服务

```bash
# 启动数据库
docker-compose up -d postgres

# 启动后端
cd backend && cp .env.example .env && cargo run

# 启动 TEE 服务 (新终端)
cd tee-service && cargo run

# 启动前端 (新终端)
cd frontend && npm install && npm run dev
```

### 测试

```bash
# 运行集成测试
./scripts/test-integration.sh
```

## 项目结构

```
tee-rwa/
├── frontend/      # Next.js 前端
├── backend/       # Rust Axum 后端
├── tee-service/   # Occlum TEE 服务
├── solana-program/# Anchor Solana 合约
└── docs/          # 设计文档
```

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Next.js + TypeScript + Tailwind |
| 后端 | Rust (Axum) |
| TEE | Occlum + Rust (Simulation Mode) |
| 区块链 | Solana (Anchor) |
| 数据库 | PostgreSQL |

## 核心功能

### 1. RA-TLS 安全通道
前端在 TEE 服务获取 Quote，验证后用 TEE 公钥加密数据上传。

### 2. 隐私信用评估
TEE 内部解密数据，执行评分算法，生成签名报告。

### 3. 链上验证
评估结果 hash + ECDSA 签名上链，任何人可公开验证。

## 密钥体系

| 密钥 | 类型 | 用途 |
|------|------|------|
| 企业钱包私钥 | Ed25519 | 身份认证、解密报告 |
| 企业钱包公钥 | Ed25519 | 加密企业版报告 |
| TEE签名私钥 | secp256k1 | 签名评估报告 |
| TEE签名公钥 | secp256k1 | 链上验证签名 |

## 开发模式

当前使用 Occlum Simulation Mode，无需真实 SGX 硬件。

生产部署需切换到云服务器 SGX 实例（阿里云/腾讯云）。

## 文档

- [设计文档](docs/superpowers/specs/2026-04-17-tee-rwa-credit-system-design.md)
- [实现计划](docs/superpowers/plans/2026-04-17-tee-rwa-credit-system.md)

## License

MIT
