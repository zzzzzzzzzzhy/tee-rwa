# TEE-RWA 隐私信用评估系统

基于 TEE 与 Solana 的供应链金融隐私信用评估系统。

## 技术栈

- 前端: Next.js + TypeScript + Tailwind
- 后端: Rust (Axum)
- TEE: Occlum + Rust
- 区块链: Solana (Anchor)
- 数据库: PostgreSQL

## 开发环境

```bash
# 启动数据库
docker-compose up -d postgres

# 启动后端
cd backend && cargo run

# 启动前端
cd frontend && npm run dev

# 启动TEE服务
cd tee-service && ./build.sh
```

## 文档

- [设计文档](docs/superpowers/specs/2026-04-17-tee-rwa-credit-system-design.md)
- [实现计划](docs/superpowers/plans/2026-04-17-tee-rwa-credit-system.md)
