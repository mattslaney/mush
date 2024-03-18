use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::hash::Hasher;

mod macros;
use clap::ValueEnum;
use walkdir::WalkDir;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum MushMode {
    /// Copy files to destination
    Copy,
    /// Move files to destination
    Move,
}

pub enum MushAction {
    Add,      //[+] Add new file to dest
    Remove,   //[-] Remove file from dest
    Skip,     //[*] Skip file from source (duplicate)
    Ignore,   //[_] Ignore file from source
    Update,   //[>] Update file on the dest
    Retreive, //[<] Retreive file from the dest
    Collision,//[!] Hash collision detected - unlikely
}

pub fn scan(src: Vec<String>, dst: String, manifest_filename: String) {
    let mut hashmap: HashMap<String, String> = HashMap::new();

    let mut manifest =
        std::fs::File::create(manifest_filename).expect("Could not create manifest file");
    if let Err(e) = writeln!(manifest, "action,hash,src,dst") {
        eprintln!("Error: {}", e);
    }

    let mut i = 0;
    let sources = src;
    for source in sources {
        let files = WalkDir::new(&source);
        for file in files {
            i += 1;
            print!("\rProcessing #{}...", i);
            std::io::stdout().flush().unwrap();
            let file = file.expect("Expected file");
            if file.path().is_file() {
                let src_parent_dir = file.path().parent().unwrap().to_str().unwrap();
                let src_file_name = file.path().file_name().unwrap().to_str().unwrap();
                let src_path_string =
                    String::from(file.path().to_str().expect("Expected file path"));
                let src_rel_path = file.path().strip_prefix(&source).unwrap();
                let pb = file.path().to_path_buf();

                let dst_dir_path_string = match dst.ends_with(std::path::MAIN_SEPARATOR) {
                    true => format!("{}", dst),
                    false => format!("{}{}", dst, std::path::MAIN_SEPARATOR),
                };

                let hash = get_file_hash(&pb, None);
                let _hash_datetime = std::time::SystemTime::now();
                let _created_date = file.metadata().unwrap().created().unwrap();
                let _modified_date = file.metadata().unwrap().modified().unwrap();

                if hashmap.contains_key(&hash) {
                    let orig = hashmap.get(&hash).unwrap();

                    //Get orig created date & modified date
                    let orig_file_path = std::path::Path::new(orig);
                    let orig_metadata = std::fs::metadata(orig_file_path).unwrap();
                    let _orig_created = orig_metadata.created().unwrap();
                    let _orig_modified = orig_metadata.modified().unwrap();

                    //Do a bit by bit file comparison
                    if compare_files(&src_path_string, orig) {
                        let s_path = style!("dim,white", "{}/{}", src_parent_dir, src_file_name);
                        println!("Skipped: {} (same as {})", s_path, orig);
                        if let Err(e) =
                            writeln!(manifest, "[*],{},{},{}", hash, src_path_string, orig)
                        {
                            eprintln!("Failed to write to duplicates file: {}", e);
                        }
                    } else {
                        let s_path = style!("yellow", "{}/{}", src_parent_dir, src_file_name);
                        println!("Collision detected: {} (same as {})", s_path, orig);
                        if let Err(e) =
                            writeln!(manifest, "[!],{},{},{}", hash, src_path_string, orig)
                        {
                            eprintln!("Failed to write to duplicates file: {}", e);
                        }
                    }
                } else {
                    let dst_path_string =
                        String::from(format!("{}{}", dst_dir_path_string, src_rel_path.display()));
                    // let s_path = style!("green", "{}", &src_path_string);
                    // let s_hash = style!("dim,white", "{}", &hash);
                    // println!("NEW: {}: {}", s_path, s_hash);
                    if let Err(e) = writeln!(
                        manifest,
                        "[+],{},{},{}",
                        &hash, &src_path_string, &dst_path_string
                    ) {
                        eprintln!("Failed to write to manifest file: {}", e);
                    }
                    hashmap.insert(hash.to_owned(), src_path_string.to_owned());
                }
            }
        }
    }
}


#[allow(dead_code)]
enum HashType {
    Seahash,
}

fn get_file_hash(path: &std::path::PathBuf, hash_type: Option<HashType>) -> String {
    let input = std::fs::File::open(path).expect("Expected to open file");
    let reader = std::io::BufReader::new(input);
    match hash_type {
        Some(HashType::Seahash) => get_seahash(reader).to_string(),
        None => get_seahash(reader).to_string(),
    }
}

fn get_seahash<R: Read>(mut reader: R) -> u64 {
    let mut hasher = seahash::SeaHasher::default();
    let mut buffer = [0; 1024];
    loop {
        let count = reader
            .read(&mut buffer)
            .expect("Expected to read from reader");
        if count == 0 {
            break;
        }
        hasher.write(&buffer[..count]);
    }

    hasher.finish()
}

fn compare_files(file1: &str, file2: &str) -> bool {
    let mut f1 = File::open(file1).expect("Expected to open file1");
    let mut f2 = File::open(file2).expect("Expected to open file2");

    let mut buffer1 = [0; 4096];
    let mut buffer2 = [0; 4096];

    let total_bytes = f1.metadata().unwrap().len();
    let mut bytes_processed = 0;
    println!("Comparing {} and {}", file1, file2);
    println!("{} bytes to compare", total_bytes);

    loop {
        let bytes_read1 = f1.read(&mut buffer1).expect("Expected to read from file1");
        let bytes_read2 = f2.read(&mut buffer2).expect("Expected to read from file2");

        if bytes_read1 == 0 && bytes_read2 != 0 {
            return false; // File1 is shorter
        }

        if bytes_read1 != 0 && bytes_read2 == 0 {
            return false; // File2 is shorter
        }

        if bytes_read1 == 0 && bytes_read2 == 0 {
            break; // Files are identical
        }

        if bytes_read1 != bytes_read2 {
            return false; // Different file sizes
        }

        if buffer1[..bytes_read1] != buffer2[..bytes_read2] {
            return false; // Bytes differ
        }

        bytes_processed += bytes_read1 as u64;
        let percent = (bytes_processed * 100) / total_bytes;
        print!("\rComparing {}%", percent);
    }

    let msg = style!("green", "Files are identical");
    println!("\r{}", msg);
    true // Files are identical
}

#[allow(dead_code, unused_variables)]
pub fn push(manifest: String, mode: MushMode) {}
