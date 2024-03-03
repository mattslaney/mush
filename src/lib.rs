pub mod sync {
    use crate::filesystem::{*};
    use std::error::Error;
    use std::path::PathBuf;

    pub fn run() -> Result<(), Box<dyn Error>> {
        let files_to_sync = crate::filesystem::get_file_sync_list();
        for file in files_to_sync {
            let (file_path, file_action) = file;
            match file_action {
                FileAction::CREATE => {
                    move_file(&file_path, None);
                    update_file_hash(&file_path.to_path_buf());
                }
                FileAction::UPDATE => {
                    move_file(&file_path, None);
                    update_file_hash(&file_path.to_path_buf());
                }
                FileAction::DELETE => {
                    let dst_file = path_src_to_dst(&file_path);
                    delete_file(&dst_file);
                    remove_obj_file(&file_path);
                }
            }
        }
        Ok(())
    }

    pub fn list() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub fn status() -> Result<(), Box<dyn Error>> {
        let files_to_sync = crate::filesystem::get_file_sync_list();
        let mut c = Vec::<PathBuf>::new();
        let mut u = Vec::<PathBuf>::new();
        let mut d = Vec::<PathBuf>::new();

        for file in files_to_sync {
            let (file_path, file_action) = file;
            match file_action {
                FileAction::CREATE => c.push(file_path),
                FileAction::UPDATE => u.push(file_path),
                FileAction::DELETE => d.push(file_path),
            }
        }

        println!("");
        println!("New Files");
        for file in c {
            println!("{:?}", file);
        }

        println!("");
        println!("Updated Files");
        for file in u {
            println!("{:?}", file);
        }

        println!("");
        println!("Deleted Files");
        for file in d {
            println!("{:?}", file);
        }

        println!("");

        Ok(())
    }

    pub fn push() -> Result<(), Box<dyn Error>> {

        Ok(())
    }
}

mod filesystem {
    use std::collections::{HashMap, HashSet};
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::hash::Hasher;
    use std::io::{BufReader, Read};
    use std::path::{Path, PathBuf};

    use serde::{Deserialize, Serialize};

    use ignore::DirEntry;

    use crate::config::{self, get_cfg_src_dir};
    use ignore::WalkBuilder;

    //Storing u64 as String because toml only supports up to i64
    #[derive(Serialize, Deserialize)]
    struct FileObject {
        file_name: String,
        rel_path: String,
        created: String,
        modified: String,
        checksum: String,
    }

    fn get_all_files() -> Vec<PathBuf> {
        let mut all_files = Vec::<PathBuf>::new();
        let src_dir = crate::config::get_cfg_src_dir().unwrap();
        let ignore_file = config::get_ignore_file().unwrap().unwrap();

        let mut builder = WalkBuilder::new(src_dir);
        builder.hidden(false);
        builder.ignore(false);
        builder.parents(false);
        builder.git_global(false);
        builder.git_ignore(false);
        builder.git_exclude(false);
        builder.require_git(false);
        builder.follow_links(false);
        builder.add_custom_ignore_filename(ignore_file);
        for item in builder.build() {
            let dir: DirEntry = item.unwrap();
            let pb: PathBuf = dir.into_path();
            match pb.is_file() {
                true => all_files.push(pb),
                false => (),
            }
        }

        all_files
    }

    //TODO::
    #[allow(dead_code)]
    fn get_src_rel_path(path: &PathBuf) -> PathBuf {
        let root_dir = config::get_cfg_src_dir().unwrap();
        path.strip_prefix(root_dir)
            .expect("Could not get relative path to source dir")
            .to_path_buf()
    }

    fn get_src_rel_path_str(path: &Path) -> String {
        path.strip_prefix(config::get_cfg_src_dir().unwrap())
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap().to_string()
    }

    fn get_src_rel_path_hash(path: &Path) -> u64 {
        let rel_path_str = get_src_rel_path_str(path);
        seahash::hash(rel_path_str.as_bytes())
    }

    fn get_obj_file(path: &PathBuf) -> PathBuf {
        let rel_path_hash = get_src_rel_path_hash(path);
        let obj_path = format!("{}/objects/{}", config::get_cfg_dir_str(), rel_path_hash);
        Path::new(&obj_path).to_path_buf()
    }

    pub fn remove_obj_file(src_file: &PathBuf) -> bool {
        let obj_file = get_obj_file(src_file);
        match fs::remove_file(obj_file) {
            Ok(_) => true,
            Err(_) => {
                eprintln!("Could not remove obj file");
                false
            }
        }
    }

    fn get_file_obj(obj_path: PathBuf) -> FileObject {
        let contents = std::fs::read_to_string(&obj_path)
            .expect(format!("Could not get file object: {:?}", &obj_path).as_str());
        let file_obj: FileObject = toml::from_str(&contents).unwrap();
        file_obj
    }

    // fn get_all_file_objects() -> Vec<FileObject> {
    //     let mut obj_files = Vec::<FileObject>::new();
    //     let cfg_dir = config::get_cfg_dir_str();
    //     let obj_dir = format!("{cfg_dir}/objects");
    //     for entry in std::fs::read_dir(obj_dir).unwrap() {
    //         let entry = entry.unwrap();
    //         obj_files.push(get_file_obj(entry.path()));
    //     }

    //     obj_files
    // }

    #[allow(dead_code)]
    fn get_all_file_object_refs() -> HashSet<String> {
        let mut obj_files = HashSet::<String>::new();
        let cfg_dir = config::get_cfg_dir_str();
        let obj_dir = format!("{cfg_dir}/objects");
        for entry in std::fs::read_dir(obj_dir).unwrap() {
            let entry = entry.unwrap();
            let checksum = String::from(entry.file_name().to_string_lossy());
            obj_files.insert(checksum);
        }

        obj_files
    }

    fn get_all_file_object_map() -> HashMap<String, String> {
        let mut obj_files = HashMap::<String, String>::new();
        let cfg_dir = config::get_cfg_dir_str();
        let obj_dir = format!("{cfg_dir}/objects");
        for entry in std::fs::read_dir(obj_dir).unwrap() {
            let entry = entry.unwrap();
            let obj = get_file_obj(entry.path());
            let checksum = String::from(entry.file_name().to_string_lossy());
            obj_files.insert(checksum, obj.rel_path);
        }

        obj_files
    }

    #[derive(Debug)]
    pub enum FileAction {
        CREATE,
        UPDATE,
        DELETE,
    }

    pub fn get_file_sync_list() -> HashMap<PathBuf, FileAction> {
        let mut changed_files = HashMap::<PathBuf, FileAction>::new();
        let mut tracked_files = get_all_file_object_map();

        get_all_files().iter().for_each(|f| {
            let rel_path_hash = get_src_rel_path_hash(f.as_path());
            //let file_name = String::from(f.file_name().unwrap().to_string_lossy());
            let new_checksum = get_file_hash(f.to_path_buf()).unwrap();

            //Compare checksums if available
            let obj_path = format!("{}/objects/{}", config::get_cfg_dir_str(), rel_path_hash);
            match std::fs::metadata(&obj_path).is_ok() {
                true => {
                    let contents = std::fs::read_to_string(&obj_path).unwrap();
                    let file_obj: FileObject = toml::from_str(&contents).unwrap();
                    let prev_checksum = file_obj.checksum.parse::<u64>().unwrap();
                    if prev_checksum != new_checksum {
                        //println!("Updated file: {}: {}", file_name, new_checksum);
                        changed_files.insert(f.to_path_buf(), FileAction::UPDATE);
                    }
                    tracked_files.remove(&rel_path_hash.to_string());
                }
                false => {
                    //println!("New file: {}: {}", file_name, new_checksum);
                    changed_files.insert(f.to_path_buf(), FileAction::CREATE);
                }
            }
        });

        //Files to delete
        for (_, src_file) in &tracked_files {
            let root_dir = get_cfg_src_dir().unwrap();
            let src_path_str = format!("{}/{}", root_dir, src_file);
            let src_path = std::path::Path::new(&src_path_str);
            //println!("Deleted file: {:?}", src_path);
            changed_files.insert(src_path.to_path_buf(), FileAction::DELETE);
        }

        changed_files
    }

    fn get_seahash<R: Read>(mut reader: R) -> Result<u64, Box<dyn Error>> {
        let mut hasher = seahash::SeaHasher::default();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.write(&buffer[..count]);
        }
        Ok(hasher.finish())
    }

    fn get_file_hash(path: PathBuf) -> Result<u64, Box<dyn Error>> {
        let input = File::open(path)?;
        let reader = BufReader::new(input);
        let hash = get_seahash(reader)?;
        Ok(hash)
    }

    pub fn update_file_hash(path: &PathBuf) {
        let rel_path_str = get_src_rel_path_str(path);
        let rel_path_hash = get_src_rel_path_hash(path);
        let file_name = String::from(path.file_name().unwrap().to_string_lossy());
        let created = path
            .metadata()
            .unwrap()
            .created()
            .unwrap()
            .elapsed()
            .unwrap();
        let modified = path
            .metadata()
            .unwrap()
            .created()
            .unwrap()
            .elapsed()
            .unwrap();
        let new_checksum = get_file_hash(path.to_path_buf()).unwrap();

        let file_obj = FileObject {
            file_name,
            rel_path: rel_path_str.to_string(),
            created: created.as_secs().to_string(),
            modified: modified.as_secs().to_string(),
            checksum: new_checksum.to_string(),
        };
        let contents = toml::to_string(&file_obj);
        match &contents {
            Ok(_) => (),
            Err(e) => {
                println!("Error: {:#?}", e)
            }
        }
        let obj_path = format!("{}/objects/{}", config::get_cfg_dir_str(), rel_path_hash);

        std::fs::write(&obj_path, contents.unwrap()).expect("Could not write object file");
    }

    pub fn path_src_to_dst(src_file: &PathBuf) -> PathBuf {
        let root_dir = config::get_cfg_src_dir().unwrap();
        let rel_path = src_file.strip_prefix(&root_dir).unwrap().to_str().unwrap();
        let dst_dir = config::get_cfg_dst_dir().unwrap();
        let dst_file = format!("{}/{}", dst_dir, rel_path);
        std::path::Path::new(dst_file.as_str()).to_path_buf()
    }

    pub fn move_file(src_file: &PathBuf, dst_file: Option<&PathBuf>) -> bool {
        let dst_file = match dst_file {
            Some(dst_file) => dst_file.to_owned(),
            None => path_src_to_dst(src_file),
        };
        let dst_path = dst_file.as_path();
        let dst_dir = dst_path.parent().unwrap();
        println!("Moving: {:?} to {:?}", src_file, dst_file);
        fs::create_dir_all(dst_dir).expect("Failed to create directory structure");
        match fs::copy(src_file, dst_file) {
            Ok(_) => true,
            Err(_) => {
                eprintln!("Failed to copy");
                false
            }
        }
    }

    pub fn delete_file(file: &PathBuf) -> bool {
        fs::remove_file(file).expect("Failed to remove file");
        if file.parent().unwrap().read_dir().unwrap().next().is_none() {
            match fs::remove_dir(file.parent().unwrap().to_path_buf()) {
                Ok(_) => true,
                Err(_) => {
                    eprintln!("Failed to remove empty directory");
                    false
                }
            }
        } else {
            false
        }
    }
}

pub mod setup {
    use std::{env, error::Error, fs};

    use crate::config;

    pub fn src_init(dst: &Option<String>, gitignore: &bool) -> Result<(), Box<dyn Error>> {
        // Create the config folder
        let dot_folder = config::get_cfg_dir_str();
        fs::create_dir(&dot_folder)?;

        // Create the config file
        let cwd = env::current_dir().expect("Could not get current working directory");
        let cwd_str = cwd
            .to_str()
            .expect("Failed to get current working dir as string");
        println!("Source directory is: {}", cwd_str);
        match dst {
            Some(v) => println!("Destination directory is: {}", v),
            None => println!("No destination is defined"),
        }

        //Make .syncignore file
        match gitignore {
            true => {
                println!(".syncignore linked to .gitignore");
                fs::hard_link(".gitignore", ".syncignore")?;
            }
            false => {
                println!(".syncignore created");
                fs::write(".syncignore", ".*")?;
            }
        }

        let config = config::Config {
            core: config::CoreConfig {
                ignore_file: Some(".syncignore".to_string()),
                mode: String::from("bidirectional"),
                src: String::from(cwd_str),
                dst: dst.to_owned(),
            },
        };

        let config_contents = toml::to_string(&config).unwrap();

        fs::write(format!("{}/config", &dot_folder), config_contents)?;

        Ok(())
    }

    pub fn src_clear_cache() -> Result<(), Box<dyn Error>> {
        let cfg_dir = config::get_cfg_dir_str();
        for entry in fs::read_dir(format!("{}/objects", cfg_dir))? {
            let entry = entry?;
            fs::remove_file(entry.path())?;
        }
        Ok(())
    }

    pub fn src_delete() -> Result<(), Box<dyn Error>> {
        let dot_folder = config::get_cfg_dir_str();
        for entry in fs::read_dir(&dot_folder)? {
            let entry = entry?;
            println!("Deleting: {:#?}", entry.file_name());
            fs::remove_file(entry.path())?;
        }
        fs::remove_dir(&dot_folder)?;
        fs::remove_file(".syncignore")?;
        Ok(())
    }
}

pub mod config {
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use std::{
        env,
        error::Error,
        fs,
        io::{BufRead, BufReader},
    };

    pub fn exists() -> bool {
        match get_cfg() {
            Ok(_) => true,
            Err(_) => false
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub core: CoreConfig,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CoreConfig {
        pub ignore_file: Option<String>,
        pub mode: String,
        pub src: String,
        pub dst: Option<String>,
    }

    #[derive(Debug, Clone)]
    pub struct ConfigError;

    impl fmt::Display for ConfigError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Failed to get config")
        }
    }

    impl std::error::Error for ConfigError {}

    fn get_cfg() -> Result<Config, ConfigError> {
        let cfg_file = get_cfg_dir_str();
        let contents = match fs::read_to_string(format!("{}/config", cfg_file)) {
            Ok(data) => data,
            Err(_) => return Err(ConfigError),
        };
        let config: Config = toml::from_str(&contents).unwrap();
        Ok(config)
    }

    fn get_exe_name() -> String {
        env::current_exe()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    pub fn get_cfg_dir_str() -> String {
        let exe_name = get_exe_name();
        let dot_folder = String::from(format!(".{}", &exe_name));
        dot_folder
    }

    pub fn get_cfg_src_dir() -> Result<String, Box<dyn Error>> {
        let cfg = get_cfg()?;
        Ok(cfg.core.src)
    }

    pub fn get_cfg_dst_dir() -> Result<String, Box<dyn Error>> {
        let cfg = get_cfg()?;
        let dst_dir = cfg.core.dst.expect("No destination directory setup");
        Ok(dst_dir)
    }

    pub fn cfg_set_dst(dst: String) -> Result<(), Box<dyn Error>> {
        let dot_folder = get_cfg_dir_str();
        let mut config = get_cfg()?;
        config.core.dst = Some(dst);
        let config_contents = toml::to_string(&config).unwrap();
        fs::write(format!("{}/config", &dot_folder), config_contents)?;
        println!("New sync destination set");
        Ok(())
    }

    pub fn get_ignore_file() -> Result<Option<String>, Box<dyn Error>> {
        let cfg = get_cfg()?;
        Ok(cfg.core.ignore_file)
    }

    pub fn get_ignore_patterns() -> Result<Vec<String>, Box<dyn Error>> {
        let mut v = Vec::<String>::new();
        let ignore_file = get_ignore_file()?;
        match ignore_file {
            Some(f) => {
                let input = fs::File::open(f)?;
                let buffered = BufReader::new(input);
                for line in buffered.lines() {
                    let line = line?;
                    v.push(line)
                }
            }
            None => (),
        };
        Ok(v)
    }
}

// struct SrcDir {
//     path: PathBuf,
//     string: String,
// }

// fn get_src_dir() -> Result<SrcDir, Box<dyn Error>> {
//     let mut cwd = SrcDir {
//         path: env::current_dir()?,
//         string: String::new(),
//     };
//     if let Some(path_str) = cwd.path.to_str() {
//         cwd.string = path_str.to_owned();
//     }
//     return Ok(cwd);
// }

// fn set_cfg(mode: String) -> Result<(), Box<dyn Error>> {
//     Ok(())
// }
