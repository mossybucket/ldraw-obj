use ahash::AHashMap;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct LDrawIndex {
    files: AHashMap<String, PathBuf>,
}

impl LDrawIndex {
    pub fn build(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();

        let mut files = AHashMap::new();

        for entry in WalkDir::new(root)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();

            let Some(ext) = path.extension() else {
                continue;
            };

            if !ext.eq_ignore_ascii_case("dat") {
                continue;
            }

            let relative = path
                .strip_prefix(root)
                .context("Invalid LDraw path")?
                .to_string_lossy()
                .replace('\\', "/")
                .to_lowercase();

            //
            // Ignore unwanted folders
            //
            if relative.starts_with("textures/") || relative.starts_with("s/") {
                continue;
            }

            //
            // Full path lookup
            //
            files.insert(relative.clone(), path.to_path_buf());

            //
            // Filename lookup
            //
            if let Some(name) = path.file_name() {
                files
                    .entry(name.to_string_lossy().to_lowercase())
                    .or_insert_with(|| path.to_path_buf());
            }
        }

        println!("Indexed {} LDraw files", files.len());

        Ok(Self { files })
    }

    pub fn resolve(&self, name: &str) -> Option<&PathBuf> {
        let mut key = name.replace('\\', "/").to_lowercase();

        //
        // Remove ./ prefix
        //
        while key.starts_with("./") {
            key = key[2..].to_string();
        }

        //
        // Exact match
        //
        if let Some(path) = self.files.get(&key) {
            return Some(path);
        }

        //
        // LDraw often uses:
        // 3001.dat
        // instead of parts/3001.dat
        //
        if let Some(file) = key.rsplit('/').next() {
            if let Some(path) = self.files.get(file) {
                return Some(path);
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &PathBuf)> {
        self.files.iter()
    }
}
