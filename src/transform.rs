use glam::{Mat3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Mat3,
}

impl Transform {
    #[inline]
    pub fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Mat3::IDENTITY,
        }
    }

    /// LDraw type-1 transform
    ///
    /// x y z a b c d e f g h i
    #[inline]
    pub fn from_ldraw(values: &[f32]) -> Self {
        assert!(values.len() >= 12, "Invalid LDraw transform");

        Self {
            translation: Vec3::new(values[0], values[1], values[2]),

            rotation: Mat3::from_cols(
                Vec3::new(values[3], values[6], values[9]),
                Vec3::new(values[4], values[7], values[10]),
                Vec3::new(values[5], values[8], values[11]),
            ),
        }
    }

    #[inline]
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.rotation * point + self.translation
    }

    /// Combine:
    /// parent * child
    #[inline]
    pub fn combine(&self, child: &Transform) -> Transform {
        Transform {
            rotation: self.rotation * child.rotation,

            translation: self.rotation * child.translation + self.translation,
        }
    }

    #[inline]
    pub fn determinant(&self) -> f32 {
        self.rotation.determinant()
    }

    #[inline]
    pub fn is_inverted(&self) -> bool {
        self.determinant() < 0.0
    }
}
