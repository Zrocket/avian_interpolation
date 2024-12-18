use std::f32::consts::TAU;

use avian3d::prelude::*;
use avian_interpolation3d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

mod util;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Disabling SyncPlugin is optional, but will get you a performance boost.
            PhysicsPlugins::default().build().disable::<SyncPlugin>(),
            AvianInterpolationPlugin::default(),
            util::plugin(util::Example::Generic),
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_box)
        .run();
}

#[derive(Component)]
struct Moving;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Name::new("Player Camera"), Camera3dBundle::default()));

    commands.spawn((
        Name::new("Light"),
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 2_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));

    let box_shape = Cuboid::from_size(Vec3::splat(1.0));
    commands.spawn((
        Name::new("Box"),

        Mesh3d(meshes.add(Mesh::from(box_shape))),
        MeshMaterial3d(materials.add(Color::from(tailwind::EMERALD_300))),
        Transform::from_xyz(0.0, 0.0, -5.),

        RigidBody::Kinematic,
        Collider::from(box_shape),
        Moving,
    ));
}

fn move_box(time: Res<Time>, mut moving: Query<&mut Position, With<Moving>>) {
    let elapsed = time.elapsed_secs();
    let max_offset = 1.7;
    let oscillations_per_second = 0.6;
    for mut position in &mut moving {
        let interpolant = elapsed * oscillations_per_second * TAU;
        position.0.x = interpolant.sin() * max_offset;
    }
}
