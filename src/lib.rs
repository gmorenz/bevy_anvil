use std::{error::Error, io::Cursor};

use anvil::Part;
use bevy::render::mesh::Mesh;
use stl::{read_stl, stl_to_triangle_mesh, stl_to_wireframe_mesh};

#[cfg(feature = "dynamic")]
#[expect(
    unused_imports,
    clippy::single_component_path_imports,
    reason = "Using to force linking."
)]
use anvil_dylib;

mod stl;

pub use anvil;

pub fn part_to_mesh(part: Part) -> Result<Mesh, Box<dyn Error>> {
    let stl_mesh = part_to_stl(part)?;
    Ok(stl_to_triangle_mesh(&stl_mesh))
}

pub fn part_to_wireframe(part: Part) -> Result<Mesh, Box<dyn Error>> {
    let stl_mesh = part_to_stl(part)?;
    Ok(stl_to_wireframe_mesh(&stl_mesh))
}

fn part_to_stl(part: Part) -> Result<stl_io::IndexedMesh, Box<dyn Error>> {
    // TODO: This is painful, it's going through a temp file to just reload the
    // data, then split it into lines, for us to merge the lines, so that we can
    // pass it to the parser.
    let stl = part
        .stl()?
        .into_iter()
        .flat_map(|s| s.into_bytes().into_iter().chain(std::iter::once(b'\n')))
        .collect::<Vec<_>>();
    Ok(read_stl(&mut Cursor::new(&stl))?)
}
