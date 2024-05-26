use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
    render::view::window,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResolution,
};
use wasm_bindgen::prelude::*;

#[derive(Resource)]
struct PreviousCursorPosition(Vec2);

#[derive(Component)]
struct Lifetime {
    timer: Timer,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#wasm_container".into()),
                resolution: WindowResolution::new(500.0, 500.0),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(DefaultPlugins)
        .insert_resource(PreviousCursorPosition(Vec2::ZERO))
        .add_systems(Startup, setup)
        .add_systems(Update, update_bloom_settings)
        .add_systems(Update, cursor_events)
        .add_systems(Update, fade_out_system)
        .run();
}

#[wasm_bindgen(start)]
pub fn wasm_start() {
    main();
}

fn setup(
    mut commands: Commands,
    // mut windows: Query<&mut Window>,
    mut clear_color: ResMut<ClearColor>,
) {
    // // Window size
    // let mut window = windows.single_mut();
    // window.resolution.set(200.0, 100.0);
    // window.canvas = Some("#wasm_animation".to_string());

    // Set the clear color to transparent
    clear_color.0 = Color::NONE;

    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

fn update_bloom_settings(mut camera: Query<(Entity, Option<&mut BloomSettings>), With<Camera>>) {
    let bloom_settings = camera.single_mut();

    match bloom_settings {
        (_, Some(mut bloom_settings)) => {
            bloom_settings.intensity = bloom_settings.intensity.clamp(0.0, 1.0);

            bloom_settings.low_frequency_boost = bloom_settings.low_frequency_boost.clamp(0.0, 1.0);

            bloom_settings.low_frequency_boost_curvature =
                bloom_settings.low_frequency_boost_curvature.clamp(0.0, 1.0);

            bloom_settings.high_pass_frequency = bloom_settings.high_pass_frequency.clamp(0.0, 1.0);

            bloom_settings.prefilter_settings.threshold =
                bloom_settings.prefilter_settings.threshold.max(0.0);

            bloom_settings.prefilter_settings.threshold_softness = bloom_settings
                .prefilter_settings
                .threshold_softness
                .clamp(0.0, 1.0);
        }

        (_, None) => {}
    }
}

fn cursor_events(
    mut commands: Commands,
    mut cursor_evr: EventReader<CursorMoved>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut previous_cursor_position: ResMut<PreviousCursorPosition>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_width = window.width();
    let window_height = window.height();
    let line_width = 8.0;
    for ev in cursor_evr.read() {
        // Draw circle at cursor
        let new_x = ev.position.x;
        let new_y = ev.position.y;

        let start_pos = Vec2::new(
            previous_cursor_position.0.x - window_width / 2.0,
            window_height / 2.0 - previous_cursor_position.0.y,
        );
        let end_pos = Vec2::new(new_x - window_width / 2.0, window_height / 2.0 - new_y);

        let direction = end_pos - start_pos;
        let length = direction.length();
        let angle = -direction.angle_between(Vec2::new(1.0, 0.0));

        let line_mesh = Mesh::from(shape::Quad::new(Vec2::new(length, line_width)));

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(line_mesh)),
                material: materials.add(Color::rgba(7.5, 0.0, 7.5, 0.2)),
                transform: Transform {
                    translation: Vec3::new(
                        (start_pos.x + end_pos.x) / 2.0,
                        (start_pos.y + end_pos.y) / 2.0,
                        0.0,
                    ),
                    rotation: Quat::from_rotation_z(angle),
                    ..Default::default()
                },
                ..Default::default()
            },
            Lifetime {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ));
        previous_cursor_position.0 = Vec2::new(new_x, new_y);
    }
}

fn fade_out_system(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime, &Handle<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, mut lifetime, material_handle) in query.iter_mut() {
        if lifetime.timer.tick(time.delta()).finished() {
            commands.entity(entity).despawn();
        } else {
            if let Some(material) = materials.get_mut(material_handle) {
                let alpha = 1.0 - lifetime.timer.fraction_remaining();
                material.color.set_a(alpha);
            }
        }
    }
}
