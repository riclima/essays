use bevy::prelude::*;

#[derive(Resource)]
struct Court {
    dimensions: Vec2,
}

#[derive(Component)]
struct Player {
    id: u8,
}

#[derive(Component)]
struct Ball;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK.into()))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, apply_velocity).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Single<&Window>,
) {
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
    let paddle_scale = Vec3::new(paddle_width, paddle_height, 1.0);
    let paddle_offset = (COURT_WIDTH / 2.0) - (paddle_width / 2.0);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(-paddle_offset, 0.0, 0.0),
            scale: paddle_scale,
            ..default()
        },
        Player { id: 0 },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(paddle_offset, 0.0, 0.0),
            scale: paddle_scale,
            ..default()
        },
        Player { id: 1 },
    ));

    // Set up the game's ball.
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(16.0, 16.0, 1.0),
            ..default()
        },
        Velocity(Vec2::new(0.5, -0.5).normalize() * 300.0),
    ));

    // Now that we are done with resources, we can insert them into the
    // world so other systems can use them.
    commands.insert_resource(Court {
        dimensions: Vec2::new(COURT_WIDTH, window.height() - (WALL_THICKNESS * 2.0)),
    });
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    court: Res<Court>,
) {
    for (mut transform, player) in &mut query {
        let mut direction = 0.0;
        let (up_key, down_key) = match player.id {
            0 => (KeyCode::KeyW, KeyCode::KeyS),
            _ => (KeyCode::ArrowUp, KeyCode::ArrowDown),
        };

        if keyboard_input.pressed(up_key) {
            direction += 1.0;
        }
        if keyboard_input.pressed(down_key) {
            direction -= 1.0;
        }

        const PADDLE_SPEED: f32 = 500.0;
        let new_y = transform.translation.y + direction * PADDLE_SPEED * time.delta_secs();
        let paddle_half_height = transform.scale.y / 2.0;
        let half_height = (court.dimensions.y / 2.0) - paddle_half_height;
        transform.translation.y = new_y.clamp(-half_height, half_height);
    }
}
