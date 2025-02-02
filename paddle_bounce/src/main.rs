use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player {
    id: u8,
}

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

const BALL_RADIUS: f32 = 12.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BLACK.into()))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_paddle, play_collision_sound).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    window: Single<&Window>,
) {
    // Set up the game's camera.
    commands.spawn(Camera2d);

    let ball_collision_sound = asset_server.load("sounds/breakout_collision.ogg");
    commands.insert_resource(CollisionSound(ball_collision_sound));

    // Set up the game's field.
    const COURT_WIDTH: f32 = 1236.0;
    const WALL_THICKNESS: f32 = 16.0;

    let offset = (window.height() / 2.0) - (WALL_THICKNESS / 2.0);
    Rectangle::default();
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, offset, 0.0),
            ..Default::default()
        },
        Collider::cuboid(COURT_WIDTH / 2.0, WALL_THICKNESS / 2.0),
    ));
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, -offset, 0.0),
            ..Default::default()
        },
        Collider::cuboid(COURT_WIDTH / 2.0, WALL_THICKNESS / 2.0),
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
    let paddle_offset = (COURT_WIDTH / 2.0) - (paddle_width / 2.0);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(paddle_width, paddle_height))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(-paddle_offset, 0.0, 0.0),
            ..default()
        },
        Player { id: 0 },
        Velocity::linear(Vec2::splat(0.0)),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::cuboid(paddle_width / 2.0, paddle_height / 2.0),
        LockedAxes::ROTATION_LOCKED_X
            | LockedAxes::ROTATION_LOCKED_Y
            | LockedAxes::ROTATION_LOCKED_Z
            | LockedAxes::TRANSLATION_LOCKED_X,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(paddle_width, paddle_height))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(paddle_offset, 0.0, 0.0),
            ..default()
        },
        Player { id: 1 },
        Velocity::linear(Vec2::splat(0.0)),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Collider::cuboid(paddle_width / 2.0, paddle_height / 2.0),
        LockedAxes::ROTATION_LOCKED_X
            | LockedAxes::ROTATION_LOCKED_Y
            | LockedAxes::ROTATION_LOCKED_Z
            | LockedAxes::TRANSLATION_LOCKED_X,
    ));

    // Set up the game's ball.
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::default(),
        RigidBody::Dynamic,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::splat(200.0)),
        Restitution::coefficient(2.0),
        GravityScale(0.0),
        Friction::new(0.0),
        ActiveEvents::COLLISION_EVENTS,
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
) {
    for (mut velocity, player) in &mut query {
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
        velocity.linvel.y = direction * PADDLE_SPEED;
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    sound: Res<CollisionSound>,
) {
    if !collision_events.is_empty() {
        collision_events.clear();
        commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
    }
}
