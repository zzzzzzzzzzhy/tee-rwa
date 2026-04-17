-- 创建 uuid-ossp 扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address VARCHAR(66) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 企业表
CREATE TABLE enterprises (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id),
    name VARCHAR(255) NOT NULL,
    license_no VARCHAR(100),
    contact VARCHAR(255),
    tee_pubkey VARCHAR(128),
    tee_pubkey_registered_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 评估记录表
CREATE TABLE evaluation_records (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    enterprise_id UUID REFERENCES enterprises(id),
    version INT NOT NULL DEFAULT 1,
    report_hash BYTEA NOT NULL,
    credit_score INT,
    risk_level INT,
    chain_tx_signature VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 加密文件元数据表
CREATE TABLE encrypted_files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    enterprise_id UUID REFERENCES enterprises(id),
    file_type VARCHAR(50) NOT NULL,
    encrypted_path VARCHAR(500) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_users_wallet ON users(wallet_address);
CREATE INDEX idx_enterprises_user ON enterprises(user_id);
CREATE INDEX idx_evaluations_enterprise ON evaluation_records(enterprise_id);
