#!/bin/bash
set -e

echo "Starting PostgreSQL..."
docker-compose up -d postgres
sleep 3

echo "Starting Backend..."
cd backend
cargo run &
BACKEND_PID=$!
cd ..

sleep 5

echo "Starting TEE Service..."
cd tee-service
cargo run &
TEE_PID=$!
cd ..

sleep 3

echo "Starting Frontend..."
cd frontend
npm run dev &
FRONTEND_PID=$!
cd ..

echo "All services started!"
echo "Frontend: http://localhost:3000"
echo "Backend: http://localhost:8080"
echo "TEE Service: http://localhost:8081"

wait
