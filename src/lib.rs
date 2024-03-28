use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::hash::Hasher;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;

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

struct MushActionError {
    message: String,
}

impl std::fmt::Display for MushActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub enum MushAction {
    Add,       //[+] Add new file to dest
    Remove,    //[-] Remove file from dest
    Skip,      //[*] Skip file from source (duplicate)
    Ignore,    //[_] Ignore file from source
    Update,    //[>] Update file on the dest
    Retreive,  //[<] Retreive file from the dest
    Collision, //[!] Hash collision detected - unlikely
}

impl MushAction {
    fn to_string(&self) -> String {
        match self {
            MushAction::Add => String::from("[+]"),
            MushAction::Remove => String::from("[-]"),
            MushAction::Skip => String::from("[*]"),
            MushAction::Ignore => String::from("[_]"),
            MushAction::Update => String::from("[>]"),
            MushAction::Retreive => String::from("[<]"),
            MushAction::Collision => String::from("[!]"),
        }
    }

    fn from_string(s: &str) -> Option<MushAction> {
        match s {
            "[+]" => Some(MushAction::Add),
            "[-]" => Some(MushAction::Remove),
            "[*]" => Some(MushAction::Skip),
            "[_]" => Some(MushAction::Ignore),
            "[>]" => Some(MushAction::Update),
            "[<]" => Some(MushAction::Retreive),
            "[!]" => Some(MushAction::Collision),
            _ => None,
        }
    }
}

pub struct MushLink {
    action: MushAction,
    hash: String,
    src: String,
    dst: String,
}

fn write_to_manifest(action: MushAction, hash: &str, src: &str, dst: &str, manifest: &mut Option<File>) {
    if let Err(e) = writeln!(manifest.as_mut().unwrap(),"{},{},{},{}", action.to_string(), hash, src, dst) {
        eprintln!("Failed to write to duplicates file: {}", e);
    }
}

pub fn scan(src: Vec<String>, dst: String, manifest_filename: Option<String>) -> HashMap<String, MushLink> {
    let mut mushmap: HashMap<String, MushLink> = HashMap::new();

    let mut manifest = match &manifest_filename {
        Some(m) => Some(
            std::fs::File::create(m)
                .expect("Could not create manifest file"),
        ),
        None => None,
    };

    if manifest_filename.is_some() {
        if let Err(e) = writeln!(manifest.as_ref().unwrap(), "action,hash,src,dst") {
            eprintln!("Error: {}", e);
        }
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
                let src_file = file.path().to_path_buf();
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

                if mushmap.contains_key(&hash) {
                    print!("Possible duplicate: ");
                    let orig = mushmap.get(&hash).unwrap();

                    //Get orig created date & modified date
                    let orig_metadata = std::fs::metadata(&orig.src).unwrap();
                    let _orig_created = orig_metadata.created().unwrap();
                    let _orig_modified = orig_metadata.modified().unwrap();

                    //Do a bit by bit file comparison
                    let dup_file = std::path::PathBuf::from(&src_file);
                    if compare_files(&src_file, &dup_file) {
                        println!(
                            "{}: {} (same as {})",
                            style!("yellow", "{}", "Skipped"),
                            src_path_string,
                            orig.src
                        );
                        write_to_manifest(MushAction::Skip, &hash[..], &src_path_string[..], &orig.dst[..], &mut manifest);
                    } else {
                        let s_path = style!("yellow", "{}/{}", src_parent_dir, src_file_name);
                        println!("Collision detected: {} {})", s_path, orig.src);
                        write_to_manifest(MushAction::Collision, &hash[..], &src_path_string[..], &orig.dst[..], &mut manifest);
                    }
                } else {
                    let dst_path_string =
                        String::from(format!("{}{}", dst_dir_path_string, src_rel_path.display()));
                    // let s_path = style!("green", "{}", &src_path_string);
                    // let s_hash = style!("dim,white", "{}", &hash);
                    // println!("NEW: {}: {}", s_path, s_hash);

                    write_to_manifest(MushAction::Add, &hash[..], &src_path_string[..], &dst_path_string[..], &mut manifest);
                    let mushlink = MushLink {
                        action: MushAction::Add,
                        hash: hash.to_owned(),
                        src: src_path_string.to_owned(),
                        dst: dst_path_string.to_owned(),
                    };
                    mushmap.insert(hash.to_owned(), mushlink);
                }
            }
        }
    }

    mushmap
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

fn compare_files(file1: &PathBuf, file2: &PathBuf) -> bool {
    let mut f1 = File::open(file1).expect("Expected to open file1");
    let mut f2 = File::open(file2).expect("Expected to open file2");

    let mut buffer1 = [0; 4096];
    let mut buffer2 = [0; 4096];

    let total_bytes = f1.metadata().unwrap().len();
    let mut bytes_processed = 0;
    print!(
        "{}",
        style!(
            "dim,white",
            "Comparing {:?} and {:?}",
            file1.file_name().unwrap(),
            file2.file_name().unwrap()
        )
    );
    println!(
        "{}",
        style!("dim,white", "{} bytes to compare", total_bytes)
    );

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
    print!("\r{}...", msg);
    true // Files are identical
}

#[allow(dead_code, unused_variables)]
pub fn push(manifest: &String, dst: Option<String>, mode: MushMode) {
    let manifest_file = File::open(manifest).expect(&format!(
        "{}: Could not open file: {}",
        style!("red", "ERROR"),
        manifest
    ));

    let reader = BufReader::new(manifest_file);
    let lines = reader.lines();

    for line in lines {
        let line = line.unwrap();
        let mut fields = line.split(',');
        let action = fields.next().expect("Line missing action");
        let hash = fields.next().expect("Line missing hash");
        let src = fields.next().expect("Line missing src");
        let dst = fields.next().expect("Line missing dst");

        match MushAction::from_string(action) {
            Some(MushAction::Add) => {
                let src_file = PathBuf::from(src);
                let dst_file = PathBuf::from(dst);
                let dst_path = dst_file.parent().unwrap();
                if let Err(e) = std::fs::create_dir_all(dst_path) {
                    eprintln!(
                        "{}: Failed to create directory: {}",
                        style!("red", "ERROR"),
                        e
                    );
                    continue;
                }
                match mode {
                    MushMode::Copy => {
                        println!(
                            "{}",
                            style!(
                                "cyan",
                                "Copying {} to {}",
                                src_file.display(),
                                dst_file.display()
                            )
                        );
                        if let Err(e) = std::fs::copy(&src_file, &dst_file) {
                            eprintln!("{}: Failed to copy file: {}", style!("red", "ERROR"), e);
                            continue;
                        }
                        println!("{}", style!("green", "Copied successfully"));
                    }
                    MushMode::Move => {
                        println!(
                            "{}",
                            style!(
                                "cyan",
                                "Moving {} to {}",
                                src_file.display(),
                                dst_file.display()
                            )
                        );
                        if let Err(e) = std::fs::rename(&src_file, &dst_file) {
                            eprintln!("{}: Failed to move file: {}", style!("red", "ERROR"), e);
                            continue;
                        }
                        println!("{}", style!("green", "Moved successfully"));
                    }
                }
            }
            Some(MushAction::Skip) => {
                println!(
                    "{}",
                    style!("dim, white", "Skipping {} as is duplicate of {}", src, dst)
                );
            }
            _ => (),
        }
    }
}
