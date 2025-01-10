use core::num;
use std::os::unix::raw::off_t;

use bevy::{color::palettes::css::{BLACK, WHITE}, image::BevyDefault, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
    // Firstly, we create the resources that we will use in our game.
    let background = ClearColor(Color::BLACK.into());

    // Set up the game's camera.
    commands.spawn(Camera2d);

    // Set up the game's field.
    const WALL_THICKNESS: f32 = 16.0;
    const COURT_WIDTH: f32 = 1236.0;

    let offset = (window.height() / 2.0) - (WALL_THICKNESS / 2.0);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, offset, 0.0),
            scale: Vec3::new(COURT_WIDTH, WALL_THICKNESS, 0.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, -offset, 0.0),
            scale: Vec3::new(COURT_WIDTH, WALL_THICKNESS, 0.0),
            ..Default::default()
        },
    ));

    const NUM_UNITS: f32 = 63.0;
    const NUM_DASHES: i32 = 16;

    let court_bound = (window.height() / 2.0) - WALL_THICKNESS;
    let dash_unit = (court_bound * 2.0) / NUM_UNITS;
    let dash_size = dash_unit * 3.0;

    for i in 0..NUM_DASHES {
        let y = i as f32 * (dash_size + dash_unit) - court_bound + (dash_size / 2.0);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform {
                translation: Vec3::new(0.0, y, 0.0),
                scale: Vec3::new(WALL_THICKNESS / 3.0, dash_size, 0.0),
                ..Default::default()
            },
        ));
    }

    // Set up the game's paddles.
    let paddle_width = 16.0;
    let paddle_height = 64.0;
    let paddle_offset = (COURT_WIDTH / 2.0) - 16.0;
    let paddle_scale = Vec3::new(paddle_width, paddle_height, 1.0);

    commands.spawn((
        Name::new("player1"),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(-paddle_offset, 0.0, 0.0),
            scale: paddle_scale,
            ..default()
        },
    ));

    commands.spawn((
        Name::new("player2"),
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(paddle_offset, 0.0, 0.0),
            scale: paddle_scale,
            ..default()
        },
    ));

    // Now that we are done with resources, we can insert them into the
    // world so other systems can use them.
    commands.insert_resource(background);
}
