#!/bin/bash
set -e

# Build the project
cargo build --quiet

PROGRAM="../target/debug/rgit"

# Setup test directory
rm -rf test_repo
mkdir test_repo
cd test_repo

# 1. Test init
echo "Testing init..."
$PROGRAM init
if [ ! -d ".git" ]; then
    echo "Error: .git directory not created"
    exit 1
fi

# 2. Test hash-object
echo "Testing hash-object..."
echo "hello world" > test.txt
HASH=$($PROGRAM hash-object -w test.txt)
echo "Hash: $HASH"
if [ -z "$HASH" ]; then
    echo "Error: hash-object failed"
    exit 1
fi

# 3. Test cat-file
echo "Testing cat-file..."
CONTENT=$($PROGRAM cat-file -p $HASH)
if [ "$CONTENT" != "hello world" ]; then
    echo "Error: cat-file output mismatch. Expected 'hello world', got '$CONTENT'"
    exit 1
fi

# 4. Test write-tree
echo "Testing write-tree..."
TREE_HASH=$($PROGRAM write-tree)
echo "Tree Hash: $TREE_HASH"
if [ -z "$TREE_HASH" ]; then
    echo "Error: write-tree failed"
    exit 1
fi

# 5. Test ls-tree
echo "Testing ls-tree..."
LS_OUTPUT=$($PROGRAM ls-tree --name-only $TREE_HASH)
if [[ "$LS_OUTPUT" != *"test.txt"* ]]; then
    echo "Error: ls-tree output mismatch. Expected to contain 'test.txt', got '$LS_OUTPUT'"
    exit 1
fi

# 6. Test commit-tree
echo "Testing commit-tree..."
# Create a dummy parent commit hash (random 40 chars)
PARENT_HASH="3b18e512dba79e4c8300dd08aeb37f8e728b8dad"
COMMIT_HASH=$($PROGRAM commit-tree $TREE_HASH -p $PARENT_HASH -m "Initial commit")
echo "Commit Hash: $COMMIT_HASH"
if [ -z "$COMMIT_HASH" ]; then
    echo "Error: commit-tree failed"
    exit 1
fi

# Verify commit object exists
if [ ! -f ".git/objects/${COMMIT_HASH:0:2}/${COMMIT_HASH:2}" ]; then
    echo "Error: Commit object file not found"
    exit 1
fi

echo "All tests passed!"
