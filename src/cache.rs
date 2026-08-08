use std::sync::Arc;

use ahash::AHashMap;
use parking_lot::RwLock;

use crate::mesh::Mesh;


pub struct MeshCache {

    meshes:
        RwLock<AHashMap<String, Arc<Mesh>>>,

}



impl MeshCache {


    pub fn new() -> Self {

        Self {

            meshes:
                RwLock::new(
                    AHashMap::new()
                ),

        }

    }



    #[inline]
    pub fn get(
        &self,
        name: &str,
    ) -> Option<Arc<Mesh>> {

        self.meshes
            .read()
            .get(name)
            .cloned()

    }



    #[inline]
    pub fn insert(
        &self,
        name: String,
        mesh: Mesh,
    ) -> Arc<Mesh> {


        let mesh =
            Arc::new(mesh);



        self.meshes
            .write()
            .entry(name)
            .or_insert_with(|| mesh.clone())
            .clone()

    }



    pub fn contains(
        &self,
        name: &str,
    ) -> bool {

        self.meshes
            .read()
            .contains_key(name)

    }



    pub fn clear(
        &self,
    ) {

        self.meshes
            .write()
            .clear();

    }



    pub fn len(
        &self,
    ) -> usize {

        self.meshes
            .read()
            .len()

    }



    pub fn memory_estimate(
        &self,
    ) -> usize {

        self.meshes
            .read()
            .values()
            .map(
                |mesh| {

                    mesh.vertices.len()
                    *
                    std::mem::size_of::<crate::mesh::Vertex>()

                    +

                    mesh.triangles.len()
                    *
                    std::mem::size_of::<crate::mesh::Triangle>()

                }
            )
            .sum()

    }

}



impl Default for MeshCache {

    fn default() -> Self {

        Self::new()

    }

}