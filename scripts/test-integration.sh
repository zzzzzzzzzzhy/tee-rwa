#!/bin/bash
set -e

echo "Testing Backend Health..."
curl -s http://localhost:8080/health || { echo "Backend not responding"; exit 1; }

echo "Testing TEE Service..."
curl -s http://localhost:8081/quote || { echo "TEE service not responding"; exit 1; }

echo "Testing Wallet Connect..."
curl -s -X POST http://localhost:8080/api/wallet/connect \
  -H "Content-Type: application/json" \
  -d '{"wallet_address":"TestWallet111111111111111111111111111"}' || { echo "Wallet connect failed"; exit 1; }

echo "All integration tests passed!"
