use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

use bevy_anvil::{
    anvil::{Axis, Cuboid, Cylinder, IntoAngle as _, IntoLength as _, Point},
    part_to_mesh,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SimpleSubsecondPlugin::default()))
        .with_hot_patch(|app: &mut App| {
            app.add_systems(StartupRerunHotPatch, (setup, add_cube));
        })
        .run();
}

#[hot(hot_patch_signature = true)]
fn setup(mut commands: Commands) {
    commands.queue(|world: &mut World| {
        // Currently bevy forgets to do `track_caller` on `commands.spawn` so to
        // auto-despawn entities spawned inside a StartupRerunHotPatch schedule
        // we need to call spawn on `world` instead.
        world.spawn((
            Camera3d::default(),
            Transform::from_xyz(30.0, 30.0, 30.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        world.spawn((
            PointLight {
                intensity: 5000.,
                ..default()
            },
            Transform::from_xyz(0.0, 10.0, 0.0),
        ));

        world.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 5000.0,
            ..default()
        });
    })
}

#[hot(hot_patch_signature = true)]
fn add_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Rerunning add_cube");

    let block_width = 16.mm();
    let block_height = 9.6.mm();
    let stud_height = 11.2.mm() - block_height;
    let stud_distance = 8.mm();
    let stud_diameter = 4.8.mm();
    let thickness = 1.2.mm();
    let tube_diameter = 6.5.mm();

    let block = Cuboid::from_dim(block_width, block_width, block_height);

    let studs = Cylinder::from_diameter(stud_diameter, stud_height)
        .move_to(Point::new([
            stud_distance / 2.,
            stud_distance / 2.,
            (block_height + stud_height) / 2.,
        ]))
        .circular_pattern(Axis::<3>::z(), 4);

    let inner_block = Cuboid::from_dim(
        block_width - thickness,
        block_width - thickness,
        block_height,
    )
    .move_to(Point::new([0.0.mm(), 0.0.mm(), thickness * -0.5]));

    let inner_tube = Cylinder::from_diameter(tube_diameter, block_height - thickness).subtract(
        &Cylinder::from_diameter(tube_diameter - thickness / 2., block_height - thickness),
    );

    let part = block
        .add(&studs)
        .subtract(&inner_block)
        .add(&inner_tube)
        .rotate_around(Axis::<3>::x(), -90.0.deg());

    let mesh = match part_to_mesh(part) {
        Ok(mesh) => mesh,
        Err(e) => {
            error!("Failed to turn part to mesh: {e:?}");
            return;
        }
    };

    let mesh = meshes.add(mesh);
    let material = materials.add(Color::srgb_u8(12, 60, 12));

    commands.queue(|world: &mut World| {
        world.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::splat(1000.)),
        ));
    });

    info!("Success");
}
