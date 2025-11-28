#!/bin/bash
set -e

# Generate TypeScript types for frontend
flatc --ts -o ../web/src/generated messages.fbs

# Generate Rust types for backend
flatc --rust -o ../rust/src/generated messages.fbs

echo "✅ FlatBuffers code generation complete"
echo "   TypeScript: ./web/src/generated/"
echo "   Rust: ./rust/src/generated/"
