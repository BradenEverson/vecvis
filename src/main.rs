//! Main Bevy Entrypoint with 3-space Graphing

use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{wireframe::WireframePlugin, DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{Camera3dBundle, Commands, Mesh, ResMut, Sphere, Transform},
    utils::default,
    DefaultPlugins,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use vecvis::vector::PointCollection;

fn main() {
    let line = |t| (t, t * t, t);
    let mut points = PointCollection::default();
    points.fill_span(line, 0..10, 0.1);

    App::new()
        .insert_resource(points)
        .add_plugins(DefaultPlugins)
        .add_plugins(InfiniteGridPlugin)
        .add_plugins(WireframePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
/// Starts up the graphing program by drawing axis lines and drawing the initial graphed function
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut points: ResMut<PointCollection>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGridBundle::default());

    let mesh_handle = meshes.add(Mesh::from(Sphere::new(0.01)));
    let material_handle = materials.add(Color::WHITE);

    let point_group: Vec<_> = points
        .into_iter()
        .map(|(x, y, z)| {
            (PbrBundle {
                mesh: mesh_handle.clone(),
                material: material_handle.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },)
        })
        .collect();

    commands.spawn_batch(point_group);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
