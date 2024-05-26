use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_xpbd_2d::prelude::*;

pub struct DemoPlugin;

impl Plugin for DemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, run_demo);
    }
}

fn run_demo(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let num_balls = 300;

    let ball_radius = 10;
    let start_x = -400;
    let start_y = 200;
    let balls_in_row = 800 / (ball_radius * 2 + 5);

    for i in 0..num_balls {
        let row = i / balls_in_row;
        let column = i % balls_in_row;
        let x = start_x + (column * (ball_radius * 2 + 5));
        let y = start_y - (row * (ball_radius * 2 + 5));
        commands.spawn((
            RigidBody::Dynamic,
            Collider::ball(ball_radius as f32),
            GravityScale(30.0),
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.)),
                ..default()
            },
            LinearVelocity(Vec2::new((i * 2) as f32, i as f32)),
        ));
    }

    // Ground
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1000.0, 20.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(1000.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., -250., 0.)),
            ..default()
        },
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 500.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(20.0, 500.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-500., 0., 0.)),
            ..default()
        },
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(20.0, 500.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(20.0, 500.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(500., 0., 0.)),
            ..default()
        },
    ));

    // Ceiling
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1000.0, 20.0),
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(1000.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 250., 0.)),
            ..default()
        },
    ));
}
