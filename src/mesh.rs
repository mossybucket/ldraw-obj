use ahash::AHashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub indices: [u32; 3],
    pub color: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,

    vertex_map: AHashMap<VertexKey, u32>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct VertexKey {
    x: i64,
    y: i64,
    z: i64,
    color: u32,
}

impl VertexKey {
    #[inline]
    fn new(x: f32, y: f32, z: f32, color: u32) -> Self {
        const SCALE: f64 = 100000.0;

        Self {
            x: (x as f64 * SCALE).round() as i64,
            y: (y as f64 * SCALE).round() as i64,
            z: (z as f64 * SCALE).round() as i64,
            color,
        }
    }
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            triangles: Vec::new(),
            vertex_map: AHashMap::new(),
        }
    }

    #[inline]
    pub fn add_vertex(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        color: u32,
    ) -> u32 {

        let key = VertexKey::new(x, y, z, color);

        if let Some(index) = self.vertex_map.get(&key) {
            return *index;
        }

        let index = self.vertices.len() as u32;

        self.vertices.push(Vertex {
            position: [x, y, z],
            color,
        });

        self.vertex_map.insert(key, index);

        index
    }


    #[inline]
    pub fn add_triangle(
        &mut self,
        a: [f32; 3],
        b: [f32; 3],
        c: [f32; 3],
        color: u32,
    ) {
        let i0 = self.add_vertex(a[0], a[1], a[2], color);
        let i1 = self.add_vertex(b[0], b[1], b[2], color);
        let i2 = self.add_vertex(c[0], c[1], c[2], color);

        self.triangles.push(Triangle {
            indices: [i0, i1, i2],
            color,
        });
    }


    pub fn add_triangle_winding(
        &mut self,
        a: [f32; 3],
        b: [f32; 3],
        c: [f32; 3],
        color: u32,
        inverted: bool,
    ) {
        if inverted {
            self.add_triangle(a, c, b, color);
        } else {
            self.add_triangle(a, b, c, color);
        }
    }


    pub fn add_quad_winding(
        &mut self,
        a: [f32; 3],
        b: [f32; 3],
        c: [f32; 3],
        d: [f32; 3],
        color: u32,
        inverted: bool,
    ) {
        if inverted {
            self.add_triangle(a, c, b, color);
            self.add_triangle(a, d, c, color);
        } else {
            self.add_triangle(a, b, c, color);
            self.add_triangle(a, c, d, color);
        }
    }


    pub fn append(&mut self, other: &Mesh) {

        let mut remap =
            Vec::with_capacity(other.vertices.len());


        for v in &other.vertices {
            remap.push(
                self.add_vertex(
                    v.position[0],
                    v.position[1],
                    v.position[2],
                    v.color,
                )
            );
        }


        for tri in &other.triangles {

            self.triangles.push(Triangle {
                indices: [
                    remap[tri.indices[0] as usize],
                    remap[tri.indices[1] as usize],
                    remap[tri.indices[2] as usize],
                ],
                color: tri.color,
            });
        }
    }
}