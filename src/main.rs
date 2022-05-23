use scan_duplicates::{compare_hashes, generate_hash, parser::Parser, Result as LazyResult};
use std::{collections::HashMap, ffi::OsStr, fs::read_dir, path::PathBuf};
use structopt::StructOpt;

fn main() -> LazyResult<()> {
    let args = Parser::from_args();
    let expanded = shellexpand::tilde(&args.source_dir);

    let mut hash_store: HashMap<PathBuf, usize> = HashMap::new();
    let mut original_store: Vec<usize> = vec![];
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
        if original_store
            .iter()
            .any(|original_hash| compare_hashes(*original_hash, *hash) >= args.match_threshold)
        {
            duplicate_store.push(path);
            continue;
        }

        original_store.push(*hash);
    }

    println!("{:?}", duplicate_store);

    Ok(())
}
