# Git in Rust

This project is a minimal implementation of Git, written in Rust. It was built to understand how Git works under the hood by rebuilding its core functionality from scratch.

## What is in the code?

The codebase implements the fundamental commands of Git. These are the low-level commands that Git uses internally to manage the repository.

Here is what each part does:

### Initialization
The `init` command sets up the `.git` directory structure. It creates the necessary folders for objects and refs, just like the real Git does when you start a new project.

### Object Storage
Git stores everything as "objects". This implementation handles three main types:

1. **Blobs**: These represent file contents. When you use `hash-object`, the program reads a file, calculates its unique SHA-1 hash, compresses it, and stores it in the `.git/objects` directory. This is how Git remembers the content of your files.

2. **Trees**: These represent directories. The `write-tree` command looks at your current directory and creates a tree object that lists all files and subdirectories, along with their permissions and hashes. This captures the state of your project's structure at a specific point in time.

3. **Commits**: These represent a snapshot in history. The `commit-tree` command takes a tree (your project state) and a parent commit (history), adds an author and a message, and saves it. This creates the history chain that makes Git so powerful.

### Inspection
To verify that things are working, there are commands to inspect these objects:
- `cat-file`: Lets you read the content of a blob object using its hash.
- `ls-tree`: Lists the contents of a tree object, showing you what files and directories it contains.

The code uses the `flate2` library for Zlib compression (because Git compresses everything) and `sha1` for calculating the hashes that identify every object.
