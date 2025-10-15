#!/bin/bash

echo "🔍 Testing Certificate Validation"
echo "=================================="

# Build the binary first (quietly)
echo "Building..."
cd /Users/brian/personal/algo-chuck/algo-chuck-toolkit
cargo build -p algo-chuck-cli --release --quiet

echo "Starting server..."
./target/release/chuck ca test &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "Testing with curl..."
curl -I --connect-timeout 5 https://127.0.0.1:8443 2>&1

echo ""
echo "Testing with openssl s_client..."
echo "Q" | openssl s_client -connect 127.0.0.1:8443 -servername 127.0.0.1 2>&1 | grep -E "(verify|certificate|Verification)"

# Clean up
kill $SERVER_PID 2>/dev/null
echo "Test complete."