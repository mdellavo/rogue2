#!/usr/bin/env bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

SESSION_NAME="rogue2-dev"

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo -e "${RED}Error: tmux is not installed${NC}"
    exit 1
fi

# Check if session exists
if ! tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
    echo -e "${YELLOW}No running session found for '$SESSION_NAME'${NC}"
    exit 0
fi

echo -e "${GREEN}Stopping Rogue2 development servers...${NC}"

# Kill the session
tmux kill-session -t "$SESSION_NAME"

echo -e "${GREEN}✓ Servers stopped${NC}"
