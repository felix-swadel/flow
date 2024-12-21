mod background;
mod color;
mod consts;
mod consts_private;
mod interaction;
mod kernel;
mod maths;
mod particle;
mod physics;
mod random;
mod ui;
mod utils;

use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{
            Extent3d,
            TextureDimension,
            TextureFormat,
            TextureUsages,
        }
    }
};

use background::Background;
use consts::{BOX_LINE_WIDTH, BOX_SIZE, PIXEL_SIZE};
use consts_private::{BOX_LINE_CENTRE, BOX_SIZE_F, IMAGE_SIZE, WINDOW_SIZE_F};
use ui::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fluid Box!".to_string(),
                resolution: WINDOW_SIZE_F.into(),
                ..Default::default()
            }),
            ..Default::default()
        }).set(ImagePlugin::default_nearest()))
        .insert_resource(FrameRateLastUpdate(0.0))
        .add_systems(Startup, (setup_scene, particle::spawn))
        .add_systems(Update, (
            (
                particle::update_densities_and_pressures,
                particle::update_positions,
                particle::update_velocities,
                particle::update_accelerations,
                particle::update_colors,
                background::update,
            ).chain(),
            interaction::keypress.run_if(input_just_pressed(KeyCode::Space)),
            ui::update_frame_rate,
        ))
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(Camera2d);

    // Set up bounding box outline.
    let vertical = meshes.add(
        Rectangle::from_size(Vec2 {
            x: BOX_LINE_WIDTH,
            y: BOX_SIZE_F.1 + 2.0 * BOX_LINE_WIDTH,
        }));
    let horizontal = meshes.add(
        Rectangle::from_size(Vec2 {
            x: BOX_SIZE_F.0 + 2.0 * BOX_LINE_WIDTH,
            y: BOX_LINE_WIDTH
        }));

    let box_color= materials.add(Color::WHITE);

    // Left side.
    commands.spawn((
        Mesh2d(vertical.clone()),
        MeshMaterial2d(box_color.clone()),
        Transform::from_xyz(
            -BOX_LINE_CENTRE.0, 0.0, 0.0
        ),
    ));
    // Right side.
    commands.spawn((
        Mesh2d(vertical.clone()),
        MeshMaterial2d(box_color.clone()),
        Transform::from_xyz(
            BOX_LINE_CENTRE.0, 0.0, 0.0
        ),
    ));
    // Bottom side.
    commands.spawn((
        Mesh2d(horizontal.clone()),
        MeshMaterial2d(box_color.clone()),
        Transform::from_xyz(
            0.0, -BOX_LINE_CENTRE.1, 0.0
        ),
    ));
    // Top side.
    commands.spawn((
        Mesh2d(horizontal.clone()),
        MeshMaterial2d(box_color.clone()),
        Transform::from_xyz(
            0.0, BOX_LINE_CENTRE.1, 0.0
        ),
    ));

    // Set up frame rate text.
    commands
        .spawn((
            Text::new("FrameRate: "),
            TextFont {
                font_size: FRAME_RATE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            FrameRateUI,
            Node {
                position_type: PositionType::Absolute,
                top: FRAME_RATE_TEXT_PADDING,
                left: FRAME_RATE_TEXT_PADDING,
                ..default()
            },
        ))
        .with_child((
            TextSpan::default(),
            TextFont {
                font_size: FRAME_RATE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

    // Set up background image texture.
    let mut image = Image::new_fill(
        Extent3d {
            width: IMAGE_SIZE.0,
            height: IMAGE_SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    commands.spawn((
        Sprite {
            image: images.add(image),
            custom_size: Some(Vec2::new(
                (BOX_SIZE.0 / PIXEL_SIZE) as f32,
                (BOX_SIZE.1 / PIXEL_SIZE) as f32,
            )),
            ..Default::default()
        },
        Transform::default().with_scale(Vec3::new(
            PIXEL_SIZE as f32,
            PIXEL_SIZE as f32,
            0.0,
        )),
        Background,
    ));
}
