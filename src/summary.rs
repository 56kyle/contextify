use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn create_directory_summary(path: PathBuf, output_path: PathBuf, depth: usize) -> io::Result<()> {
    if !path.is_dir() {
        // More descriptive error message
        let err_msg = format!("The provided path '{}' is not a directory.", path.display());
        eprintln!("{}", err_msg);
        return Err(io::Error::new(io::ErrorKind::Other, err_msg));
    }

    let mut file = File::create(output_path)?;

    // Using iterators and chain methods for conciseness and efficiency
    WalkDir::new(&path)
        .max_depth(depth) // Using MAX for recursive
        .into_iter()
        .filter_map(|e| e.ok()) // Handling errors in WalkDir iteration
        .try_for_each(|entry| {
            let depth = entry.depth();
            writeln!(file, "{}{}", " ".repeat(depth * 2), entry.file_name().to_string_lossy())
        })
}

