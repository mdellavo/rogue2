#!/usr/bin/env bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Session name
SESSION_NAME="rogue2-dev"

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo -e "${RED}Error: tmux is not installed${NC}"
    echo "Install it with: brew install tmux"
    exit 1
fi

# Check if session already exists
if tmux has-session -t "$SESSION_NAME" 2>/dev/null; then
    echo -e "${YELLOW}Session '$SESSION_NAME' already exists${NC}"
    echo "Attaching to existing session..."
    tmux attach-session -t "$SESSION_NAME"
    exit 0
fi

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

echo -e "${GREEN}Starting Rogue2 development servers...${NC}"
echo -e "Backend: ${YELLOW}http://localhost:8080${NC}"
echo -e "Frontend: ${YELLOW}http://localhost:3000${NC}"
echo ""
echo -e "Press ${GREEN}Ctrl+B${NC} then ${GREEN}Up/Down arrow${NC} to switch panes"
echo -e "Press ${GREEN}Ctrl+B${NC} then ${GREEN}D${NC} to detach (servers keep running)"
echo -e "Press ${GREEN}Ctrl+C${NC} in each pane to stop servers"
echo ""
sleep 2

# Create a new tmux session
tmux new-session -d -s "$SESSION_NAME" -n "servers"

# Split the window horizontally (backend on top, frontend on bottom)
tmux split-window -v -t "$SESSION_NAME:0"

# Resize panes to be equal
tmux select-layout -t "$SESSION_NAME:0" even-vertical

# Run backend server in the top pane (pane 0)
tmux send-keys -t "$SESSION_NAME:0.0" "cd '$SCRIPT_DIR/rust'" C-m
tmux send-keys -t "$SESSION_NAME:0.0" "echo '🦀 Starting Rust backend server...'" C-m
tmux send-keys -t "$SESSION_NAME:0.0" "RUST_LOG=info cargo run" C-m

# Run frontend dev server in the bottom pane (pane 1)
tmux send-keys -t "$SESSION_NAME:0.1" "cd '$SCRIPT_DIR/web'" C-m
tmux send-keys -t "$SESSION_NAME:0.1" "echo '⚛️  Starting Vite frontend dev server...'" C-m
tmux send-keys -t "$SESSION_NAME:0.1" "npm run dev" C-m

# Select the top pane (backend) as the active pane
tmux select-pane -t "$SESSION_NAME:0.0"

# Attach to the session
tmux attach-session -t "$SESSION_NAME"
