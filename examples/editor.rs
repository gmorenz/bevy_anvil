use bevy::prelude::*;
use bevy_simple_subsecond_system::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SimpleSubsecondPlugin::default()))
        .add_systems(Startup, (setup, add_cube))
        .run();
}

#[derive(Component)]
struct Setup;
#[hot(rerun_on_hot_patch = true)]
fn setup(previous: Query<Entity, With<Setup>>, mut commands: Commands) {
    for e in previous.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        Setup,
    ));

    commands.spawn((
        PointLight {
            intensity: 500000.,
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
        Setup,
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.0,
        ..default()
    });
}

#[derive(Component)]
struct AddCube;
#[hot(rerun_on_hot_patch = true)]
fn add_cube(
    mut commands: Commands,
    previous: Query<Entity, With<AddCube>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for e in previous.iter() {
        commands.entity(e).despawn();
    }

    commands.spawn((
        AddCube,
        Mesh3d(meshes.add(Cuboid::new(3.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(0, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
}
