use bevy::window::{PresentMode, WindowMode, WindowResolution};
use bevy::{
    prelude::*,
    render::{camera::Viewport, view::RenderLayers},
    window::PrimaryWindow,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

const RENDER_LAYER_ZOOM: usize = 1;
const RENDER_LAYER_MINIMAP: usize = 2;
const WORLD_SIZE: u32 = 100000;
const MINIMAP_SCREEN_PERCENTAGE: f32 = 20.0;

#[derive(Component)]
struct Zoom;

fn zoom_config(screen_width: u32, screen_height: u32) -> (Viewport, OrthographicProjection) {
    let scale = WORLD_SIZE as f32 / screen_width as f32 * 0.3;
    (
        Viewport {
            physical_position: UVec2::new(0, 0), //
            physical_size: UVec2::new(screen_width, screen_height),
            ..default()
        },
        OrthographicProjection {
            near: 0.0,
            far: 1500.0,
            scale,
            ..default()
        },
    )
}

fn minimap_config(screen_width: u32, screen_height: u32) -> (Viewport, OrthographicProjection) {
    let map_screen_size = (screen_width as f32 * MINIMAP_SCREEN_PERCENTAGE / 100.0) as u32;
    (
        Viewport {
            physical_position: UVec2::new(screen_width - map_screen_size, 0),
            physical_size: UVec2::new(map_screen_size, map_screen_size),
            ..default()
        },
        OrthographicProjection {
            near: 0.0,
            far: 1500.0,
            scale: WORLD_SIZE as f32 / map_screen_size as f32,
            ..default()
        },
    )
}

fn setup(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = server.load("tex1.png");
    let texture_handle2: Handle<Image> = server.load("tex2.png");
    let window = q_window.single();

    let (viewport, projection) = zoom_config(
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                order: 1,
                viewport: Some(viewport),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO.with_z(100.0)),
            projection,
            camera_2d: Camera2d {},
            ..default()
        },
        RenderLayers::layer(RENDER_LAYER_ZOOM),
        Name::new("Zoom Camera"),
        Zoom,
    ));

    let (viewport, projection) = minimap_config(
        window.resolution.physical_width(),
        window.resolution.physical_height(),
    );
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                is_active: true,
                order: 2,
                viewport: Some(viewport),
                ..default()
            },
            transform: Transform::from_translation(Vec3::ZERO.with_z(200.0)),
            projection,
            camera_2d: Camera2d {},
            ..default()
        },
        RenderLayers::layer(RENDER_LAYER_MINIMAP),
        Name::new("Minimap Camera"),
    ));

    let mut rnd = ChaCha8Rng::seed_from_u64(10);

    let x = -5000.0;
    (0..10).for_each(|row| {
        (0..10).for_each(|col| {
            new_sprite(
                &mut commands,
                if rnd.gen::<u8>() % 2 == 0 {
                    &texture_handle
                } else {
                    &texture_handle2
                },
                x + (col as f32 * 1600.0),
                row as f32 * 1600.0,
            );
        });
    });
}

fn new_sprite(commands: &mut Commands, texture_handle: &Handle<Image>, x: f32, y: f32) {
    let scale = 800f32 * 2f32 / 512f32;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite { ..default() },
            texture: texture_handle.clone(),
            visibility: Visibility::Visible,
            transform: Transform::from_scale(Vec3 {
                x: scale,
                y: scale,
                z: 1f32,
            })
                .with_translation(Vec3::new(x, y, -280.0)),
            ..default()
        },
        RenderLayers::from_layers(&[RENDER_LAYER_ZOOM, RENDER_LAYER_MINIMAP]),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(2560.0, 1440.0),
                present_mode: PresentMode::AutoVsync,
                mode: WindowMode::Windowed,
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, control)
        .run();
}

fn control(input: Res<ButtonInput<KeyCode>>, mut q_transform: Query<&mut Transform, With<Zoom>>) {
    let mut transform = q_transform.single_mut();
    let step = 20.0;
    for key in input.get_pressed() {
        match key {
            KeyCode::KeyW => {
                *transform = Transform::from_translation(
                    transform.translation.with_y(transform.translation.y + step),
                );
            }
            KeyCode::KeyS => {
                *transform = Transform::from_translation(
                    transform.translation.with_y(transform.translation.y - step),
                );
            }
            KeyCode::KeyA => {
                *transform = Transform::from_translation(
                    transform.translation.with_x(transform.translation.x - step),
                );
            }
            KeyCode::KeyD => {
                *transform = Transform::from_translation(
                    transform.translation.with_x(transform.translation.x + step),
                );
            }
            KeyCode::KeyB => {
                *transform = Transform::from_translation(Vec3::new(
                    13120.0,
                    2020.0,
                    100.0,
                ));
            }
            _ => (),
        }
    }
    // dbg!(transform);
}
