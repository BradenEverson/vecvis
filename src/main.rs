//! Main Bevy Entrypoint with 3-space Graphing

use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    math::Vec3,
    pbr::{DirectionalLightBundle, PbrBundle, StandardMaterial},
    prelude::{Camera3dBundle, Commands, Cuboid, Mesh, ResMut, Transform},
    utils::default,
    DefaultPlugins,
};
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use vecvis::vector::PointCollection;

fn main() {
    let helix = |t| (f32::cos(t), f32::sin(t), t);
    let mut helix_points = PointCollection::from_fn(helix);
    helix_points.fill_span(0..10, 0.1);

    App::new()
        .insert_resource(helix_points)
        .add_plugins(DefaultPlugins)
        .add_plugins(InfiniteGridPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[allow(unused_attributes)]
#[allow(elided_lifetimes_in_paths)]
/// Starts up the graphing program by drawing axis lines and drawing the initial graphed function
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(InfiniteGridBundle::default());

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(3.0, 3.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
