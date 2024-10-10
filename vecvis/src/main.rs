//! Main Bevy Entrypoint with 3-space Graphing

use bevy::{
    app::{App, Startup, Update}, asset::Assets, color::Color, input::mouse::{MouseScrollUnit, MouseWheel}, math::Vec3, pbr::{wireframe::WireframePlugin, DirectionalLightBundle, PbrBundle, StandardMaterial}, prelude::{Camera3d, Camera3dBundle, Commands, EventReader, Mesh, Query, ResMut, Sphere, Transform, With}, utils::default, DefaultPlugins
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
        .add_systems(Update, camera_scroll)
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

fn camera_scroll(mut scroll_events: EventReader<MouseWheel>, mut query: Query<&mut Transform, With<Camera3d>>) {
    for event in scroll_events.read() {
        let scroll_amount = match event.unit {
            MouseScrollUnit::Line => event.y * -0.2,
            MouseScrollUnit::Pixel => event.y * -0.01,
        };

        for mut transform in query.iter_mut() {
            let forward = transform.rotation * Vec3::Z;
            transform.translation += forward * scroll_amount;
        }
    }
}
