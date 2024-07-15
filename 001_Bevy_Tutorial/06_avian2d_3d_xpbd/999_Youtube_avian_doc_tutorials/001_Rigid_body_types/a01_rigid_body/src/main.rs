use avian3d::{
    collision::Collider,
    dynamics::rigid_body::{AngularVelocity, RigidBody},
    PhysicsPlugins,
};

use bevy::{
    app::{App, Startup},
    asset::Assets,
    color::Color,
    core_pipeline::core_3d::Camera3dBundle,
    ecs::system::{Commands, ResMut},
    math::{
        primitives::{Cuboid, Cylinder},
        Vec3,
    },
    pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial},
    prelude::default,
    render::mesh::Mesh,
    transform::components::Transform,
    DefaultPlugins,
};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // 땅 (둥그런 모양)
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(4.0, 0.1),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(4.0, 0.1)),
            material: materials.add(Color::WHITE),
            ..Default::default()
        },
    ));

    // 빛
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // 가운데 있는 큐브
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));
}
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, setup)
        .run();
}
