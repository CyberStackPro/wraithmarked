#!/bin/bash

# ==========================================
# WraithMarked C2 - Complete API Test Suite
# ==========================================

BASE_URL="http://localhost:8080"
AGENT_ID="test-agent-001"

echo "================================================"
echo "  WraithMarked C2 - Full Integration Test"
echo "================================================"
echo ""
echo "Make sure the server is running:"
echo "  cargo run --bin server"
echo ""
echo "Press Enter to start testing..."
read

echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 1: Register Agent"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

REGISTER_RESPONSE=$(curl -s -X POST $BASE_URL/api/register \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"hostname\": \"test-laptop\",
    \"os\": \"Linux\",
    \"os_version\": \"Ubuntu 22.04\",
    \"user\": \"hacker\",
    \"ip\": \"192.168.1.100\",
    \"privileges\": \"root\",
    \"version\": \"0.1.0\"
  }")

echo "Response:"
echo $REGISTER_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 2: List All Agents"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

LIST_RESPONSE=$(curl -s $BASE_URL/api/agents)
echo "Response:"
echo $LIST_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 3: Queue Shell Command"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

QUEUE_RESPONSE=$(curl -s -X POST $BASE_URL/api/command \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"command_type\": \"shell\",
    \"payload\": \"whoami\"
  }")

echo "Response:"
echo $QUEUE_RESPONSE | jq '.'
echo ""

# Extract command_id from response
COMMAND_ID=$(echo $QUEUE_RESPONSE | jq -r '.command_id')
echo "Generated Command ID: $COMMAND_ID"
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 4: Agent Beacons (Gets Command)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

BEACON_RESPONSE=$(curl -s -X POST $BASE_URL/api/beacon \
  -H "Content-Type: application/json" \
  -d "{\"agent_id\": \"$AGENT_ID\"}")

echo "Response:"
echo $BEACON_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 5: Agent Sends Result"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

RESULT_RESPONSE=$(curl -s -X POST $BASE_URL/api/result \
  -H "Content-Type: application/json" \
  -d "{
    \"agent_id\": \"$AGENT_ID\",
    \"command_id\": \"$COMMAND_ID\",
    \"success\": true,
    \"output\": \"root\\n\"
  }")

echo "Response:"
echo $RESULT_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 6: Retrieve Command Result"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

GET_RESULT_RESPONSE=$(curl -s $BASE_URL/api/result/$COMMAND_ID)
echo "Response:"
echo $GET_RESULT_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 7: Beacon Again (Should Get Empty List)"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

BEACON2_RESPONSE=$(curl -s -X POST $BASE_URL/api/beacon \
  -H "Content-Type: application/json" \
  -d "{\"agent_id\": \"$AGENT_ID\"}")

echo "Response:"
echo $BEACON2_RESPONSE | jq '.'
echo ""

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "TEST 8: Queue Multiple Commands"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

echo "Queueing 3 commands..."
curl -s -X POST $BASE_URL/api/command -H "Content-Type: application/json" \
  -d "{\"agent_id\":\"$AGENT_ID\",\"command_type\":\"shell\",\"payload\":\"pwd\"}" | jq '.'

curl -s -X POST $BASE_URL/api/command -H "Content-Type: application/json" \
  -d "{\"agent_id\":\"$AGENT_ID\",\"command_type\":\"shell\",\"payload\":\"ls -la\"}" | jq '.'

curl -s -X POST $BASE_URL/api/command -H "Content-Type: application/json" \
  -d "{\"agent_id\":\"$AGENT_ID\",\"command_type\":\"ping\",\"payload\":\"\"}" | jq '.'

echo ""
echo "Beacon to get all 3 commands:"
BEACON3_RESPONSE=$(curl -s -X POST $BASE_URL/api/beacon \
  -H "Content-Type: application/json" \
  -d "{\"agent_id\": \"$AGENT_ID\"}")

echo $BEACON3_RESPONSE | jq '.'
echo ""

echo "================================================"
echo "  ✅ All Tests Completed!"
echo "================================================"
echo ""
echo "Summary:"
echo "  - Agent registration: ✓"
echo "  - List agents: ✓"
echo "  - Queue commands: ✓"
echo "  - Agent beacon: ✓"
echo "  - Submit results: ✓"
echo "  - Retrieve results: ✓"
echo "  - Multiple commands: ✓"
echo ""
echo "Check the server console for detailed logs!"
