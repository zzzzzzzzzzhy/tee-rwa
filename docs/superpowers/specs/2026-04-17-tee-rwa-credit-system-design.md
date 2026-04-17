# 基于 TEE 与 Solana 的供应链金融隐私信用评估系统

## 项目概述

### 目标
构建一个简历展示项目，展示隐私计算 + 区块链技术栈深度，用于求职隐私计算岗位。

### 核心功能
基于 TEE 的企业隐私身份认证与资产信用评估。让企业在不泄露商业合同细节的前提下，通过 TEE 产生可信证明，并在 Solana 上实现可验证。

### 技术亮点
- RA-TLS 安全通道（密钥由硬件保护，后端全程只处理密文）
- Occlum TEE 远程证明
- Solana 链上 ECDSA 签名验证
- SGX Sealing 密钥持久化
- 双向加密报告分发

---

## 技术栈

| 层级 | 技术 | 说明 |
|------|------|------|
| 前端 | Next.js + TypeScript + Tailwind | Web界面，企业上传材料、查看报告 |
| 后端 | Rust (Axum) | API服务，只搬运密文数据 |
| TEE | Occlum + Rust (Simulation模式) | 隐私计算核心，后续部署到SGX服务器 |
| 区块链 | Solana + Anchor | 链上验证与存证 |
| 数据库 | PostgreSQL | 业务数据存储 |
| 存储 | 本地加密存储 | 加密文件存储 |

---

## 系统架构

```
┌────────────────────────────────────────────────────────────────────────────┐
│                            用户层                                           │
│                      Web Frontend (Next.js)                                │
│   - 企业注册/登录（钱包连接）                                                │
│   - RA-TLS握手（获取TEE Quote）                                             │
│   - 合同/财报上传（TEE公钥加密）                                             │
│   - 信用评估报告查看                                                        │
│   - 链上证明验证                                                            │
└───────────────────────────┬────────────────────────────────────────────────┘
                            │ HTTPS
                            ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                         应用服务层                                          │
│                       Backend API (Rust Axum)                              │
│   - 用户认证授权                                                            │
│   - 密文数据搬运（无法解密）                                                 │
│   - 转发请求到TEE服务                                                       │
│   - 与Solana链交互                                                          │
└───────────────────────────┬────────────────────────────────────────────────┘
                            │ gRPC/本地调用
                            ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                          TEE服务层                                          │
│                    Occlum Enclave (Rust)                                   │
│   - RA-TLS握手（生成Quote + 公钥）                                          │
│   - 硬件内生成/恢复根密钥（Sealing）                                         │
│   - 解密敏感数据                                                            │
│   - 执行信用评分算法                                                        │
│   - 生成ECDSA签名报告                                                       │
│   - 双向加密报告分发                                                        │
└───────────────────────────┬────────────────────────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────────────────────────┐
│                        链上验证层                                           │
│                      Solana Smart Contract (Anchor)                        │
│   - 企业TEE公钥注册                                                         │
│   - ECDSA签名验证                                                           │
│   - 存储信用评估结果hash                                                    │
│   - 提供公开验证接口                                                        │
└────────────────────────────────────────────────────────────────────────────┘

┌────────────────────────────────────────────────────────────────────────────┐
│                        数据存储层                                           │
│                      PostgreSQL + 本地加密存储                              │
│   - 用户信息                                                                │
│   - Sealed密钥文件                                                          │
│   - 加密的系统存档报告                                                      │
│   - 评估记录                                                                │
└────────────────────────────────────────────────────────────────────────────┘
```

---

## 密钥体系概览

本项目涉及两套独立的密钥体系，面试时需清晰区分：

| 密钥 | 类型 | 体系 | 所有者 | 用途 |
|------|------|------|--------|------|
| 企业钱包私钥 | Ed25519 | Solana | 企业用户 | 身份认证、签署交易、解密企业版报告 |
| 企业钱包公钥 | Ed25519 | Solana | 公开 | 链上身份标识、加密企业版报告 |
| TEE签名私钥 | secp256k1 | TEE | TEE硬件 | 签名评估报告 |
| TEE签名公钥 | secp256k1 | TEE | 公开 | 验证报告签名、链上注册 |
| TEE主密钥 | AES-256 | TEE | TEE硬件 | 加密用户上传数据、Sealing内部密钥 |

**两套密钥的交互场景：**

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                        企业版报告加密流程                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   TEE生成报告 ──▶ 用企业Solana公钥(Ed25519)加密 ──▶ 企业版报告              │
│                          │                                                  │
│                          ▼                                                  │
│               企业用Solana私钥解密查看                                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────────┐
│                        报告签名与验证流程                                    │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│   TEE用secp256k1私钥签名 ──▶ signature                                      │
│              │                                                              │
│              ▼                                                              │
│   链上: ecdsa_verify(tee_secp256k1_pubkey, report_hash, signature)          │
│                                                                             │
│   注意: Solana原生用Ed25519，但支持secp256k1预编译指令验证ECDSA签名          │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

**面试要点：**
- 企业钱包(E25519)用于身份+加密，是Solana原生方案
- TEE签名(secp256k1)用于可验证计算证明，是TEE生态常用曲线
- 两条曲线不互通，但Solana支持secp256k1验证预编译

---

## 核心业务流程

### Phase 1: 安全通道建立 (RA-TLS)

```
┌─────────────┐                              ┌─────────────────────────────┐
│  Web前端    │  ① 请求远程证明(Quote)       │      TEE (Occlum)           │
│             │ ───────────────────────────▶ │                             │
│             │                              │ 硬件内生成:                  │
│             │ ② 返回 Quote + secp256k1公钥 │  - 私钥(永不离开TEE)         │
│             │ ◀─────────────────────────── │  - 生成Quote绑定公钥          │
│             │                              │                             │
│             │ ③ 本地验证Quote:             │                             │
│             │   - 检查QUOTE签名(Intel)     │                             │
│             │   - 验证公钥绑定              │                             │
│             │   - 检查MRENCLAVE/MRSIGNER   │                             │
│             │                              │                             │
│             │ ④ 用TEE公钥加密敏感数据       │                             │
│             │ ───────────────────────────▶ │                             │
└─────────────┘                              └─────────────────────────────┘
```

**安全保证：**
- 密钥在TEE硬件内生成，永不离开
- 后端API全程只搬运密文，无法触及明文
- Quote绑定公钥，防止中间人替换

### Phase 2: TEE内评估

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              TEE (Occlum)                                  │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │  ⑤ 硬件内解密数据                                                    │   │
│  │  ⑥ 执行信用评估算法                                                  │   │
│  │     - 分析合同关键条款                                               │   │
│  │     - 计算财务指标                                                   │   │
│  │     - 生成信用评分 (0-1000)                                          │   │
│  │  ⑦ 生成评估报告 + ECDSA签名                                          │   │
│  │     - signature = sign(report_hash, tee_private_key)                │   │
│  │  ⑧ 双向加密报告分发                                                  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────────────┘
```

### Phase 3: 链上验证与存储

```
┌─────────────────┐                          ┌─────────────────────┐
│   Backend API   │  ⑨ 提交链上交易          │   Solana Contract   │
│   (只搬运数据)  │ ─────────────────────▶   │                     │
│                 │                          │ verify_ecdsa(       │
│                 │                          │   tee_pubkey,       │
│                 │                          │   report_hash,      │
│                 │                          │   signature         │
│                 │                          │ )                   │
│                 │                          │                     │
│                 │                          │ 存储:               │
│                 │                          │ - version           │
│                 │                          │ - report_hash       │
│                 │                          │ - tee_pubkey        │
│                 │                          │ - timestamp         │
│                 │ ◀─────────────────────   │ - verification_result│
└─────────────────┘                          └─────────────────────┘
```

### Phase 4: 公开验证

任何第三方可验证：
1. 从Solana读取 tee_pubkey + report_hash + signature
2. 本地验证ECDSA签名
3. 可选: 获取Quote验证TEE身份

---

## 数据结构

### 链上账户结构

```rust
// Solana CreditReport 账户
struct CreditReportAccount {
    version: u8,                    // 版本号，支持平滑升级
    enterprise_pubkey: Pubkey,      // 企业Solana公钥
    report_hash: [u8; 32],          // 报告SHA256摘要
    credit_score: u32,              // 信用分数 (0-1000)
    risk_level: u8,                 // 风险等级
    tee_pubkey: [u8; 64],           // TEE的secp256k1公钥
    signature: [u8; 64],            // ECDSA签名
    timestamp: i64,                 // 评估时间
    verified: bool,                 // 验证通过标志
}
// 总约 180 bytes, 租金约 0.001 SOL/年
```

### 数据库结构

```sql
-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY,
    wallet_address VARCHAR(66) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 企业表
CREATE TABLE enterprises (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    license_no VARCHAR(100),
    contact VARCHAR(255),
    tee_pubkey_registered_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 评估记录表
CREATE TABLE evaluation_records (
    id UUID PRIMARY KEY,
    enterprise_id UUID REFERENCES enterprises(id),
    version INT NOT NULL DEFAULT 1,
    report_hash BYTEA NOT NULL,
    credit_score INT,
    risk_level INT,
    chain_tx_signature VARCHAR(100),
    created_at TIMESTAMP DEFAULT NOW()
);

-- 加密文件元数据表
CREATE TABLE encrypted_files (
    id UUID PRIMARY KEY,
    enterprise_id UUID REFERENCES enterprises(id),
    file_type VARCHAR(50) NOT NULL,  -- 'contract', 'financial_report'
    encrypted_path VARCHAR(500) NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- 注意: Sealed密钥不存数据库，存本地文件系统
-- 路径: /var/lib/tee-rwa/sealed/master_key.sealed
-- 原因: Sealing绑定物理机，存DB会被DBA复制走尝试离线攻击
```

---

## 密钥管理

### TEE密钥生命周期

```
TEE首次启动
    │
    ▼
TEE内部生成根密钥
- AES-256 主密钥 (加密用户数据)
- secp256k1 签名密钥对 (签名报告)
    │
    ▼
SGX Sealing: 用硬件密钥加密根密钥
sealed_key = seal(root_key)
    │
    ▼
存储到本地文件系统
路径: /var/lib/tee-rwa/sealed/master_key.sealed
权限: 仅TEE进程可读
安全: 只有同一物理机+同一Enclave能解封
    │
    ▼
后续重启: TEE读取sealed_key
root_key = unseal(sealed_key)
继续服务
```

**为什么不存数据库？**
- Sealing的核心保证是"只有同一物理机+同一Enclave能解封"
- 如果存PostgreSQL，DBA可以复制sealed blob到其他机器尝试攻击
- 存本地文件系统，攻击者需要先攻破物理机才能拿到文件
- 即使拿到文件，没有正确Enclave也无法解封

### 密钥用途汇总

| 密钥 | 类型 | 用途 | 存储位置 | 持有者 |
|------|------|------|---------|--------|
| TEE主密钥 | AES-256 | 加密用户上传数据 | Sealed本地文件，TEE内存 | TEE |
| TEE签名私钥 | secp256k1 | 签名评估报告 | Sealed本地文件，TEE内存 | TEE |
| TEE签名公钥 | secp256k1 | 验证签名，链上注册 | 公开 | 公开 |
| 企业钱包私钥 | Ed25519 | 身份认证、解密报告 | 企业钱包 | 企业用户 |
| 企业钱包公钥 | Ed25519 | 加密企业版报告 | 链上公开 | 公开 |

---

## 报告存储策略

### 双向加密方案

```
                    ┌─────────────────────┐
                    │   TEE 生成报告      │
                    └──────────┬──────────┘
                               │
                ┌──────────────┴──────────────┐
                │                             │
                ▼                             ▼
    ┌─────────────────────┐       ┌─────────────────────┐
    │ 系统存档版本        │       │ 企业用户版本        │
    │                     │       │                     │
    │ 密钥: TEE secp256k1 │       │ 密钥: 企业Solana    │
    │       私钥加密      │       │   Ed25519公钥加密   │
    │ 存储: 本地加密文件  │       │ 存储: 返回前端      │
    │ 用途: 审计/司法鉴定 │       │ 用途: 企业自己查看  │
    │                     │       │                     │
    │ report_sys.enc      │       │ report_user.enc     │
    └─────────────────────┘       └─────────────────────┘
```

**企业版报告解密流程：**
1. 企业前端收到 report_user.enc
2. 用户用Solana钱包私钥(Ed25519)解密
3. 展示评估报告内容

### 隐私保证

- 后端API：无法解密任何数据
- 企业用户：用自己钱包私钥解密查看报告
- 审计场景：TEE可用自己的私钥解密系统存档
- 链上：只存储hash，不暴露报告内容

---

## 开发环境

### 本地开发配置

| 组件 | 配置 | 说明 |
|------|------|------|
| TEE | Occlum Simulation Mode | 无需SGX硬件，纯软件模拟 |
| Solana | Devnet | 测试网络，空投获取测试币 |
| 数据库 | PostgreSQL (Docker) | 本地容器运行 |

### 生产部署配置

| 组件 | 配置 | 说明 |
|------|------|------|
| TEE | 云服务器 SGX 实例 | 阿里云/腾讯云 SGX 虚拟机 |
| Solana | Mainnet/Devnet | 根据需求选择 |
| 数据库 | 云数据库 | 高可用配置 |

---

## 目录结构

```
tee-rwa/
├── frontend/                    # Next.js 前端
│   ├── app/
│   ├── components/
│   ├── lib/
│   └── package.json
│
├── backend/                     # Rust Axum 后端
│   ├── src/
│   │   ├── api/
│   │   ├── models/
│   │   ├── services/
│   │   └── main.rs
│   └── Cargo.toml
│
├── tee-service/                 # Occlum TEE 服务
│   ├── src/
│   │   ├── crypto/
│   │   ├── evaluation/
│   │   ├── attestation/
│   │   └── main.rs
│   ├── Occlum.yaml
│   └── Cargo.toml
│
├── solana-program/              # Anchor Solana 合约
│   ├── programs/
│   │   └── credit-report/
│   ├── tests/
│   └── Anchor.toml
│
├── docs/
│   └── superpowers/
│       └── specs/
│           └── 2026-04-17-tee-rwa-credit-system-design.md
│
├── docker-compose.yml
└── README.md
```

---

## 面试展示要点

### 技术深度

1. **RA-TLS 安全通道**：密钥由硬件保护，后端完全无法触碰明文
2. **SGX Sealing**：解决TEE重启丢密钥问题
3. **ECDSA链上验证**：轻量级验证方案，避免Quote验证高Gas消耗
4. **双向加密分发**：平衡隐私保护与业务需求

### 架构亮点

1. 三层分离架构，职责清晰
2. 后端"零信任"设计（只能搬运密文）
3. 链上数据最小化（只存hash）
4. 版本控制支持平滑升级

### 可扩展讨论

1. 如需更去中心化验证，可引入预言机
2. 如需零知识证明，可在TEE内部叠加ZK
3. 如需多方安全计算，可扩展为MPC架构

---

## 面试常见问题

### Q1: 为什么用两套不同的密钥体系？

- **Ed25519 (Solana)**：Solana原生曲线，Gas高效，用于链上身份和交易签名
- **secp256k1 (TEE)**：Intel SGX/Occlum生态常用，远程证明标准曲线
- **桥梁**：Solana有secp256k1预编译，可验证TEE签名，两条曲线在此场景下可协同

### Q2: 为什么Sealed密钥存本地不存数据库？

- Sealing保证：只有同一物理机+同一Enclave能解封
- 存数据库：DBA可复制sealed blob尝试离线攻击
- 存本地：攻击者需先攻破物理机才能拿到文件
- 即使拿到，没有正确Enclave也无法解封

### Q3: 后端API为什么碰不到明文？

- RA-TLS握手在前端和TEE之间直接完成
- 数据用TEE公钥加密，只有TEE私钥能解
- 后端只负责搬运密文blob，全程无解密能力

### Q4: 企业版报告怎么加密的？

- TEE用企业的Solana公钥(Ed25519)加密报告
- 企业用自己钱包私钥解密查看
- 后端看不到明文，只搬运加密后的blob
