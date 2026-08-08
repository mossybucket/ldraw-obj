use anyhow::Result;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::mesh::Mesh;

pub fn export_obj(mesh: &Mesh, path: impl AsRef<Path>) -> Result<()> {
    let file = File::create(path)?;

    let mut writer = BufWriter::new(file);
    
    for vertex in &mesh.vertices {
        writeln!(
            writer,
            "v {} {} {}",
            vertex.position[0], 
            vertex.position[1], 
            -vertex.position[2],
        )?;
    }

    for triangle in &mesh.triangles {
        writeln!(
            writer,
            "f {} {} {}",
            triangle.indices[0] + 1,
            triangle.indices[2] + 1,
            triangle.indices[1] + 1,
        )?;
    }

    Ok(())
}
