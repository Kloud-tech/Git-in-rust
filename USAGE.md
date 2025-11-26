# How to Run the Code

This guide explains how to use this custom Git implementation and run the tests.

## Prerequisites

You need to have Rust installed on your machine.

## Building the Project

First, build the project to generate the executable:

```bash
cargo build
```

The executable will be located at `./target/debug/rgit`.

## Running Commands

You can run the commands directly using `cargo run` or by calling the executable.

### Initialize a Repository
```bash
cargo run -- init
```

### Hash a File (Create a Blob)
```bash
echo "hello world" > test.txt
cargo run -- hash-object -w test.txt
```
This will print the SHA-1 hash of the object.

### Read a Blob
```bash
cargo run -- cat-file -p <hash>
```
Replace `<hash>` with the hash you got from the previous step.

### Write a Tree
```bash
cargo run -- write-tree
```
This saves the current directory structure as a tree object and prints its hash.

### Inspect a Tree
```bash
cargo run -- ls-tree --name-only <tree-hash>
```

### Create a Commit
```bash
cargo run -- commit-tree <tree-hash> -p <parent-hash> -m "Your commit message"
```

## Running Tests

There is a verification script included that runs through all the commands to ensure they work correctly.

To run the tests:

1. Make sure the script is executable:
   ```bash
   chmod +x verify.sh
   ```

2. Run the script:
   ```bash
   ./verify.sh
   ```

This script will:
- Build the project.
- Create a temporary test directory.
- Run all implemented commands in sequence.
- Verify that the output matches expected values.
- Clean up after itself (mostly).
