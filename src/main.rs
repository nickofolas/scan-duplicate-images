use scan_duplicates::{compare_hashes, generate_hash, parser::Parser, Result as LazyResult};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{read_dir, remove_file, write},
    path::PathBuf,
    time::SystemTime,
};
use structopt::StructOpt;

fn main() -> LazyResult<()> {
    let args = Parser::from_args();
    let expanded = shellexpand::tilde(&args.source_dir);

    let mut hash_store: HashMap<PathBuf, usize> = HashMap::new();
    let mut original_store: HashMap<usize, String> = HashMap::new();
    let mut duplicate_store: Vec<&PathBuf> = vec![];

    for entry_wrapped in read_dir(expanded.to_string())? {
        let entry = entry_wrapped?;

        let file_path = match entry
            .path()
            .extension()
            .unwrap_or_else(|| OsStr::new("USELESS"))
            .to_str()
            .unwrap()
        {
            "jpg" | "jpeg" | "png" => entry.path(),
            _ => continue,
        };
        println!("Analyzing {}", file_path.to_str().unwrap());

        let hash = generate_hash(file_path.as_path())?;
        hash_store.insert(file_path, hash);
    }

    for (path, hash) in &hash_store {
        if original_store.iter().any(|(original_hash, original_path)| {
            let matches = compare_hashes(*original_hash, *hash) >= args.match_threshold;

            return match matches {
                true => {
                    println!("{:?} matches \"{}\"", path, original_path);
                    true
                }
                false => false,
            };
        }) {
            duplicate_store.push(path);
            continue;
        }

        original_store.insert(*hash, String::from(path.to_str().unwrap()));
    }

    if args.delete_files {
        let mut deleted: Vec<String> = vec![];
        for path in &duplicate_store {
            remove_file(*path)?;
            deleted.push(String::from(path.to_str().unwrap()))
        }

        println!("\nFiles deleted:\n{}", deleted.join("\n"));
    }

    if args.store_matches {
        let save_string = duplicate_store
            .iter()
            .map(|path| path.to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join("\n");

        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        write(format!("{}.txt", timestamp), save_string)?;
    }

    Ok(())
}
