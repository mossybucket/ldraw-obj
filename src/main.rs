mod cache;
mod index;
mod loader;
mod mesh;
mod obj;
mod parser;
mod transform;

use anyhow::Result;
use rayon::prelude::*;
use std::{env, fs, path::PathBuf, sync::Arc};

use cache::MeshCache;
use index::LDrawIndex;
use loader::Loader;
use obj::export_obj;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!(
            "Usage:\n\
             ldraw_converter <ldraw_folder> <output_folder>"
        );

        return Ok(());
    }

    let ldraw_path = PathBuf::from(&args[1]);

    let output_path = PathBuf::from(&args[2]);

    fs::create_dir_all(&output_path)?;

    println!("Indexing LDraw...");

    let index = Arc::new(LDrawIndex::build(&ldraw_path)?);

    println!("Found {} files", index.len());

    let cache = Arc::new(MeshCache::new());

    let loader = Arc::new(Loader::new(Arc::clone(&index), Arc::clone(&cache)));

    let files: Vec<String> = index
        .iter()
        .filter_map(|(name, _)| {
            if !name.ends_with(".dat") {
                return None;
            }

            let file = name.rsplit('/').next().unwrap_or(name);

            if file.contains('p') {
                return None;
            }

            if name.contains("/s/") {
                return None;
            }

            if name.contains("parts/") {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect();

    println!("Converting {} parts...", files.len());

    files.par_iter().for_each(|name| match loader.load(name) {
        Ok(mesh) => {
            let mut output = output_path.clone();

            output.push(name);

            output.set_extension("obj");

            if let Some(parent) = output.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    eprintln!("{} folder failed: {}", name, e);

                    return;
                }
            }

            if let Err(e) = export_obj(&mesh, output) {
                eprintln!("{} export failed: {}", name, e);
            }
        }

        Err(e) => {
            eprintln!("{} failed: {}", name, e);
        }
    });

    println!("Done. Cached meshes: {}", cache.len());

    Ok(())
}
