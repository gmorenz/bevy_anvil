use std::{error::Error, io::Cursor};

use anvil::Part;
use bevy::render::mesh::Mesh;
use stl::{read_stl, stl_to_triangle_mesh};

mod stl;

pub fn part_to_mesh(part: Part) -> Result<Mesh, Box<dyn Error>> {
    // TODO: This is painful, it's going through a temp file to just reload the
    // data, then split it into lines, for us to merge the lines, so that we can
    // pass it to the parser.
    let stl = part
        .stl()?
        .into_iter()
        .flat_map(|s| s.into_bytes().into_iter().chain(std::iter::once(b'\n')))
        .collect::<Vec<_>>();

    let stl_mesh = read_stl(&mut Cursor::new(&stl))?;

    Ok(stl_to_triangle_mesh(&stl_mesh))
}
