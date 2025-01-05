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

    // Now that we are done with resources, we can insert them into the
    // world so other systems can use them.
    commands.insert_resource(background);
}
