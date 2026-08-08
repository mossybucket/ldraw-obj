use std::{collections::HashSet, sync::Arc};

use anyhow::{Context, Result};
use glam::Vec3;

use crate::{
    cache::MeshCache,
    index::LDrawIndex,
    mesh::Mesh,
    parser::{Instruction, Parser},
    transform::Transform,
};

pub struct Loader {
    pub index: Arc<LDrawIndex>,
    pub cache: Arc<MeshCache>,
}

impl Loader {
    pub fn new(index: Arc<LDrawIndex>, cache: Arc<MeshCache>) -> Self {
        Self { index, cache }
    }

    pub fn load(&self, name: &str) -> Result<Arc<Mesh>> {
        let mut stack = HashSet::new();

        self.load_recursive(name, &mut stack)
    }

    fn load_recursive(&self, name: &str, stack: &mut HashSet<String>) -> Result<Arc<Mesh>> {
        let key = name.replace('\\', "/").to_lowercase();

        if !stack.insert(key.clone()) {
            anyhow::bail!("Circular dependency detected: {}", name);
        }

        let result = (|| {
            if let Some(mesh) = self.cache.get(&key) {
                return Ok(mesh);
            }

            let path = self
                .index
                .resolve(&key)
                .with_context(|| format!("Missing LDraw file {}", name))?;

            let data = std::fs::read_to_string(path)?;

            let mut parser = Parser::new();

            let instructions = parser.parse(&data)?;

            let mut mesh = Mesh::new();

            for instruction in instructions {
                match instruction {
                    Instruction::Triangle {
                        color,
                        points,
                        inverted,
                    } => {
                        mesh.add_triangle_winding(points[0], points[1], points[2], color, inverted);
                    }

                    Instruction::Quad {
                        color,
                        points,
                        inverted,
                    } => {
                        mesh.add_quad_winding(
                            points[0], points[1], points[2], points[3], color, inverted,
                        );
                    }

                    Instruction::SubFile {
                        position,
                        matrix,
                        file,
                        ..
                    } => {
                        let transform = Transform::from_ldraw(&[
                            position[0],
                            position[1],
                            position[2],
                            matrix[0],
                            matrix[1],
                            matrix[2],
                            matrix[3],
                            matrix[4],
                            matrix[5],
                            matrix[6],
                            matrix[7],
                            matrix[8],
                        ]);

                        let child = self.load_recursive(&file, stack)?;

                        let transformed = transform_mesh(&child, transform);

                        mesh.append(&transformed);
                    }
                }
            }

            Ok(self.cache.insert(key.clone(), mesh))
        })();

        stack.remove(&key);

        result
    }
}

fn transform_mesh(source: &Mesh, transform: Transform) -> Mesh {
    let mut output = Mesh::new();

    let inverted = transform.is_inverted();

    for triangle in &source.triangles {
        let a = transform
            .transform_point(Vec3::from(
                source.vertices[triangle.indices[0] as usize].position,
            ))
            .to_array();

        let b = transform
            .transform_point(Vec3::from(
                source.vertices[triangle.indices[1] as usize].position,
            ))
            .to_array();

        let c = transform
            .transform_point(Vec3::from(
                source.vertices[triangle.indices[2] as usize].position,
            ))
            .to_array();

        if inverted {
            output.add_triangle(a, c, b, triangle.color);
        } else {
            output.add_triangle(a, b, c, triangle.color);
        }
    }

    output
}
