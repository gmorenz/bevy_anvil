use std::error::Error;

use anvil::Part;
use bevy::{
    asset::RenderAssetUsages,
    math::{Vec2, Vec3},
    render::mesh::{Indices, Mesh, PrimitiveTopology},
};

#[cfg(feature = "dynamic")]
#[expect(
    unused_imports,
    clippy::single_component_path_imports,
    reason = "Using to force linking."
)]
use anvil_dylib;

pub use anvil;

pub fn part_to_mesh(part: Part) -> Result<Mesh, Box<dyn Error>> {
    let indexed_mesh = part.triangulate();

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // TODO: Figure out how to generate some sort of reasonable uv's, maybe?
    // For now this is what bevy_stl did so it's good enough to not break
    // anything too badly.
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        indexed_mesh
            .uvs
            .iter()
            .map(|uv| uv.map(|coord: f64| coord as f32))
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        indexed_mesh
            .vertices
            .iter()
            .map(|uv| uv.map(|coord: f64| coord as f32))
            .collect::<Vec<_>>(),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        indexed_mesh
            .normals
            .iter()
            .map(|uv| uv.map(|coord: f64| coord as f32))
            .collect::<Vec<_>>(),
    );
    mesh.insert_indices(Indices::U32(
        indexed_mesh
            .indices
            .into_iter()
            .flat_map(|ids| ids.map(|i| i as u32))
            .collect::<Vec<_>>(),
    ));

    Ok(mesh)
}

pub fn part_to_wireframe(part: Part) -> Result<Mesh, Box<dyn Error>> {
    let indexed_mesh = part.triangulate();

    let mut mesh = Mesh::new(PrimitiveTopology::LineList, RenderAssetUsages::default());

    let vertices = indexed_mesh
        .vertices
        .into_iter()
        .map(|v| Vec3::from_array(v.map(|coord: f64| coord as f32)))
        .collect::<Vec<_>>();

    // Create line indices by connecting each edge of each triangle
    let mut indices = Vec::with_capacity(indexed_mesh.indices.len() * 6); // 3 edges * 2 vertices per edge
    for triangle in &indexed_mesh.indices {
        // Connect each edge of the triangle
        for j in 0..3 {
            indices.push(triangle[j] as u32);
            indices.push(triangle[(j + 1) % 3] as u32);
        }
    }

    // Simple normals for wireframe (all pointing in same direction)
    let normals = vec![Vec3::X; vertices.len()];

    // UV coordinates (same as in part_to_mesh)
    let uvs = vec![Vec2::ZERO; vertices.len()];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    Ok(mesh)
}
