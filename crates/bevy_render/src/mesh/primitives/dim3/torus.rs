use super::super::circle_iterator::*;
use bevy_math::{primitives::Torus, Vec3};
use wgpu::PrimitiveTopology;

use crate::{
    mesh::{Indices, Mesh, Meshable},
    render_asset::RenderAssetUsages,
};

/// A builder used for creating a [`Mesh`] with a [`Torus`] shape.
#[derive(Clone, Copy, Debug)]
pub struct TorusMeshBuilder {
    /// The [`Torus`] shape.
    pub torus: Torus,
    /// The number of vertices used for each circular segment
    /// in the ring or tube of the torus.
    ///
    /// The default is `24`.
    pub minor_resolution: usize,
    /// The number of segments used for the main ring of the torus.
    ///
    /// A resolution of `4` would make the torus appear rectangular,
    /// while a resolution of `32` resembles a circular ring.
    ///
    /// The default is `32`.
    pub major_resolution: usize,
}

impl Default for TorusMeshBuilder {
    fn default() -> Self {
        Self {
            torus: Torus::default(),
            minor_resolution: 24,
            major_resolution: 32,
        }
    }
}

impl TorusMeshBuilder {
    /// Creates a new [`TorusMeshBuilder`] from an inner and outer radius.
    ///
    /// The inner radius is the radius of the hole, and the outer radius
    /// is the radius of the entire object.
    #[inline]
    pub fn new(inner_radius: f32, outer_radius: f32) -> Self {
        Self {
            torus: Torus::new(inner_radius, outer_radius),
            ..Default::default()
        }
    }

    /// Sets the number of vertices used for each circular segment
    /// in the ring or tube of the torus.
    #[inline]
    pub const fn minor_resolution(mut self, resolution: usize) -> Self {
        self.minor_resolution = resolution;
        self
    }

    /// Sets the number of segments used for the main ring of the torus.
    ///
    /// A resolution of `4` would make the torus appear rectangular,
    /// while a resolution of `32` resembles a circular ring.
    #[inline]
    pub const fn major_resolution(mut self, resolution: usize) -> Self {
        self.major_resolution = resolution;
        self
    }

    /// Builds a [`Mesh`] according to the configuration in `self`.
    pub fn build(&self) -> Mesh {
        // code adapted from http://apparat-engine.blogspot.com/2013/04/procedural-meshes-torus.html

        let n_vertices = (self.major_resolution + 1) * (self.minor_resolution + 1);
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(n_vertices);

        let major_iter = CircleIterator::new(self.major_resolution)
            .cycle()
            .take(self.major_resolution + 1);
        let minor_circle: Vec<_> = CircleIterator::new(self.minor_resolution)
            .cycle()
            .take(self.minor_resolution + 1)
            .collect();

        for (segment, theta) in major_iter.enumerate() {
            for (side, phi) in minor_circle.iter().enumerate() {
                let position = Vec3::new(
                    theta.x * (self.torus.major_radius + self.torus.minor_radius * phi.x),
                    self.torus.minor_radius * phi.y,
                    theta.y * (self.torus.major_radius + self.torus.minor_radius * phi.x),
                );

                let center = Vec3::new(
                    self.torus.major_radius * theta.x,
                    0.,
                    self.torus.major_radius * theta.y,
                );
                let normal = (position - center).normalize();

                positions.push(position.into());
                normals.push(normal.into());
                uvs.push([
                    segment as f32 / self.major_resolution as f32,
                    side as f32 / self.minor_resolution as f32,
                ]);
            }
        }

        let n_faces = (self.major_resolution) * (self.minor_resolution);
        let n_triangles = n_faces * 2;
        let n_indices = n_triangles * 3;

        let mut indices: Vec<u32> = Vec::with_capacity(n_indices);

        let n_vertices_per_row = self.minor_resolution + 1;
        for segment in 0..self.major_resolution {
            for side in 0..self.minor_resolution {
                let lt = side + segment * n_vertices_per_row;
                let rt = (side + 1) + segment * n_vertices_per_row;

                let lb = side + (segment + 1) * n_vertices_per_row;
                let rb = (side + 1) + (segment + 1) * n_vertices_per_row;

                indices.push(lt as u32);
                indices.push(rt as u32);
                indices.push(lb as u32);

                indices.push(rt as u32);
                indices.push(rb as u32);
                indices.push(lb as u32);
            }
        }

        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_indices(Indices::U32(indices))
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}

impl Meshable for Torus {
    type Output = TorusMeshBuilder;

    fn mesh(&self) -> Self::Output {
        TorusMeshBuilder {
            torus: *self,
            ..Default::default()
        }
    }
}

impl From<Torus> for Mesh {
    fn from(torus: Torus) -> Self {
        torus.mesh().build()
    }
}

impl From<TorusMeshBuilder> for Mesh {
    fn from(torus: TorusMeshBuilder) -> Self {
        torus.build()
    }
}
