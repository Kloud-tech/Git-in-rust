 #[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use flate2::read::ZlibDecoder;
#[allow(unused_imports)]
use flate2::write::ZlibEncoder;
#[allow(unused_imports)]
use flate2::Compression;
#[allow(unused_imports)]
use std::io::Read;
#[allow(unused_imports)]
use std::io::Write;
use sha1::{Digest, Sha1};

fn create_blob(content: &[u8], write: bool) -> String {
    let header = format!("blob {}\0", content.len());
    let mut full_content = header.as_bytes().to_vec();
    full_content.extend_from_slice(content);

    let mut hasher = Sha1::new();
    hasher.update(&full_content);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    if write {
        let (dir, file) = hash_hex.split_at(2);
        let path = format!(".git/objects/{}", dir);
        fs::create_dir_all(&path).ok();

        let full_path = format!("{}/{}", path, file);
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&full_content).unwrap();
        let compressed = encoder.finish().unwrap();
        fs::write(full_path, compressed).unwrap();
    }

    hash_hex
}

fn write_tree_for_dir(dir_path: &Path) -> String {
    let mut entries = Vec::new();

    let mut dir_entries: Vec<_> = fs::read_dir(dir_path)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    
    // Sort by name
    dir_entries.sort_by_key(|e| e.file_name());

    for entry in dir_entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        // Skip .git directory
        if name == ".git" {
            continue;
        }

        if path.is_dir() {
            // Recursively write tree for subdirectory
            let tree_hash = write_tree_for_dir(&path);
            let mode = "40000"; // directory mode
            entries.push((mode.to_string(), name, tree_hash));
        } else {
            // Write blob for file
            let content = fs::read(&path).unwrap();
            let blob_hash = create_blob(&content, true);
            let mode = "100644"; // regular file mode
            entries.push((mode.to_string(), name, blob_hash));
        }
    }

    // Build tree object content
    let mut tree_content = Vec::new();
    for (mode, name, hash) in entries {
        tree_content.extend_from_slice(mode.as_bytes());
        tree_content.push(b' ');
        tree_content.extend_from_slice(name.as_bytes());
        tree_content.push(0);
        // Decode hex hash to binary (20 bytes)
        let hash_bytes = hex::decode(&hash).unwrap();
        tree_content.extend_from_slice(&hash_bytes);
    }

    // Create tree object with header
    let header = format!("tree {}\0", tree_content.len());
    let mut full_content = header.as_bytes().to_vec();
    full_content.extend_from_slice(&tree_content);

    // Calculate SHA-1
    let mut hasher = Sha1::new();
    hasher.update(&full_content);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    // Write to .git/objects
    let (dir, file) = hash_hex.split_at(2);
    let path = format!(".git/objects/{}", dir);
    fs::create_dir_all(&path).ok();

    let full_path = format!("{}/{}", path, file);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_content).unwrap();
    let compressed = encoder.finish().unwrap();
    fs::write(full_path, compressed).unwrap();

    hash_hex
}

fn write_commit(tree_sha: &str, parent_sha: &str, message: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timezone = "+0000";
    let author = format!("Author Name <author@example.com> {} {}", timestamp, timezone);
    let committer = format!("Committer Name <committer@example.com> {} {}", timestamp, timezone);

    let mut content = String::new();
    content.push_str(&format!("tree {}\n", tree_sha));
    content.push_str(&format!("parent {}\n", parent_sha));
    content.push_str(&format!("author {}\n", author));
    content.push_str(&format!("committer {}\n", committer));
    content.push_str("\n");
    content.push_str(message);
    content.push_str("\n");

    let header = format!("commit {}\0", content.len());
    let mut full_content = header.as_bytes().to_vec();
    full_content.extend_from_slice(content.as_bytes());

    let mut hasher = Sha1::new();
    hasher.update(&full_content);
    let hash = hasher.finalize();
    let hash_hex = hex::encode(hash);

    let (dir, file) = hash_hex.split_at(2);
    let path = format!(".git/objects/{}", dir);
    fs::create_dir_all(&path).ok();

    let full_path = format!("{}/{}", path, file);
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&full_content).unwrap();
    let compressed = encoder.finish().unwrap();
    fs::write(full_path, compressed).unwrap();

    hash_hex
}

fn main() {
    // eprintln!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No command provided");
        return;
    }

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").ok();
            fs::create_dir(".git/objects").ok();
            fs::create_dir(".git/refs").ok();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory");
        }
        "cat-file" => {
            if args.len() < 4 || args[2] != "-p" {
                println!("Usage: cat-file -p <hash>");
                return;
            }

            let hash = &args[3];
            if hash.len() < 3 {
                eprintln!("Invalid hash: {}", hash);
                return;
            }

            let (dir, file) = hash.split_at(2);
            let path = format!(".git/objects/{}/{}", dir, file);

            if !Path::new(&path).exists() {
                eprintln!("Blob not found: {}", hash);
                return;
            }

            let content = fs::read(&path).unwrap();
            let mut decoder = ZlibDecoder::new(&content[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).unwrap();

            let nul_pos = decompressed.iter().position(|&b| b == 0).unwrap();
            let content_only = &decompressed[nul_pos + 1..];
            print!("{}", String::from_utf8_lossy(content_only));
        }
        "ls-tree" => {
            if args.len() < 4 || args[2] != "--name-only" {
                println!("Usage: ls-tree --name-only <tree_sha>");
                return;
            }

            let hash = &args[3];
            if hash.len() < 3 {
                eprintln!("Invalid hash: {}", hash);
                return;
            }

            let (dir, file) = hash.split_at(2);
            let path = format!(".git/objects/{}/{}", dir, file);

            if !Path::new(&path).exists() {
                eprintln!("Tree not found: {}", hash);
                return;
            }

            let content = fs::read(&path).unwrap();
            let mut decoder = ZlibDecoder::new(&content[..]);
            let mut decompressed = Vec::new();
            decoder.read_to_end(&mut decompressed).unwrap();

            // Skip header "tree <size>\0"
            let nul_pos = decompressed.iter().position(|&b| b == 0).unwrap();
            let mut data = &decompressed[nul_pos + 1..];

            // Parse tree entries: <mode> <name>\0<20-byte-sha>
            while !data.is_empty() {
                // Find space after mode
                let space_pos = data.iter().position(|&b| b == b' ').unwrap();
                
                // Find null byte after name
                let nul_pos = data.iter().position(|&b| b == 0).unwrap();
                
                // Extract name (between space and null)
                let name = String::from_utf8_lossy(&data[space_pos + 1..nul_pos]);
                println!("{}", name);
                
                // Skip to next entry (past the 20-byte SHA)
                data = &data[nul_pos + 1 + 20..];
            }
        }
        "write-tree" => {
            let tree_hash = write_tree_for_dir(Path::new("."));
            println!("{}", tree_hash);
        }
        "hash-object" => {
            if args.len() < 4 || args[2] != "-w" {
                println!("Usage: hash-object -w <file>");
                return;
            }
            let file_path = &args[3];
            let content = fs::read(file_path).unwrap();
            let hash = create_blob(&content, true);
            println!("{}", hash);
        }
        "commit-tree" => {
            // Usage: commit-tree <tree_sha> -p <parent_sha> -m <message>
            if args.len() < 7 {
                println!("Usage: commit-tree <tree_sha> -p <parent_sha> -m <message>");
                return;
            }
            let tree_sha = &args[2];
            let parent_sha = &args[4];
            let message = &args[6];
            let commit_hash = write_commit(tree_sha, parent_sha, message);
            println!("{}", commit_hash);
        }
        _ => {
            println!("unknown command: {}", args[1]);
        }
    }
}
