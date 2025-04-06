use avian2d::prelude::*;
use bevy::prelude::*;
use std::f32::consts::PI;

enum Court {
    Left,
    Right,
}

#[derive(Component)]
struct Player {
    court: Court,
}

#[derive(Resource, Deref)]
struct CollisionSound(Handle<AudioSource>);

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

const BALL_RADIUS: f32 = 12.0;
const BALL_SPEED: f32 = 500.0;
const COURT_WIDTH: f32 = 1236.0;
const MAX_BOUNCE_ANGLE: f32 = 75.0 * PI / 180.0;
const NUM_DASHES: i32 = 16;
const NUM_UNITS: f32 = 63.0;
const PADDLE_HEIGHT: f32 = 64.0;
const PADDLE_OFFSET: f32 = (COURT_WIDTH / 2.0) - (PADDLE_WIDTH / 2.0);
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_WIDTH: f32 = 16.0;
const WALL_THICKNESS: f32 = 16.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(PhysicsPlugins::default());

    app.insert_resource(ClearColor(Color::BLACK.into()));
    app.insert_resource(Gravity(Vec2::ZERO));

    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, move_paddle);
    app.add_systems(FixedUpdate, play_collision_sound);
    app.add_systems(FixedUpdate, paddle_bump);

    app.run();
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

    let offset = (window.height() / 2.0) - (WALL_THICKNESS / 2.0);
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, offset, 0.0),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::rectangle(COURT_WIDTH, WALL_THICKNESS),
    ));
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Rectangle::new(COURT_WIDTH, WALL_THICKNESS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, -offset, 0.0),
            ..Default::default()
        },
        RigidBody::Static,
        Collider::rectangle(COURT_WIDTH, WALL_THICKNESS),
    ));

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
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(-PADDLE_OFFSET, 0.0, 0.0),
            ..default()
        },
        Player { court: Court::Left },
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(PADDLE_WIDTH, PADDLE_HEIGHT))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(PADDLE_OFFSET, 0.0, 0.0),
            ..default()
        },
        Player {
            court: Court::Right,
        },
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
    ));

    // Set up the game's ball.
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::default(),
        RigidBody::Dynamic,
        Collider::circle(BALL_RADIUS),
        LinearVelocity(Vec2::ONE * BALL_SPEED),
        LinearDamping(0.0),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombine::Max,
        },
        Friction {
            static_coefficient: 0.0,
            dynamic_coefficient: 0.0,
            combine_rule: CoefficientCombine::Min,
        },
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    window: Single<&Window>,
) {
    for (mut transform, player) in &mut query {
        let mut direction = 0.0;
        let (up_key, down_key) = match player.court {
            Court::Left => (KeyCode::KeyW, KeyCode::KeyS),
            _ => (KeyCode::ArrowUp, KeyCode::ArrowDown),
        };

        if keyboard_input.pressed(up_key) {
            direction += 1.0;
        }
        if keyboard_input.pressed(down_key) {
            direction -= 1.0;
        }

        transform.translation.y += direction * PADDLE_SPEED * time.delta_secs();
        let max_y = window.height() / 2.0 - WALL_THICKNESS - (PADDLE_HEIGHT / 2.0);
        transform.translation.y = transform.translation.y.clamp(-max_y, max_y)
    }
}

fn paddle_bump(
    paddles: Query<(Entity, &Player, &Transform)>,
    mut ball: Single<(Entity, &Transform, &mut LinearVelocity), With<Ball>>,
    mut collision_events: EventReader<Collision>,
) {
    for Collision(contacts) in collision_events.read() {
        if contacts.entity1 == ball.0 || contacts.entity2 == ball.0 {
            let other_entity = if contacts.entity1 == ball.0 {
                contacts.entity2
            } else {
                contacts.entity1
            };

            if let Ok((_, player, paddle_transform)) = paddles.get(other_entity) {
                let relative_intersect_y = paddle_transform.translation.y - ball.1.translation.y;
                let normalized_relative_intersection = relative_intersect_y / (PADDLE_HEIGHT / 2.0);
                let bounce_angle = normalized_relative_intersection * MAX_BOUNCE_ANGLE;
                let direction = match player.court {
                    Court::Left => 1.0,
                    Court::Right => -1.0,
                };

                ball.2.x = direction * BALL_SPEED * bounce_angle.cos();
                ball.2.y = BALL_SPEED * -bounce_angle.sin();
            }
        }
    }
}

fn play_collision_sound(
    mut commands: Commands,
    mut collision_events: EventReader<Collision>,
    ball: Single<Entity, With<Ball>>,
    sound: Res<CollisionSound>,
) {
    for Collision(contacts) in collision_events.read() {
        if contacts.entity1 == *ball || contacts.entity2 == *ball {
            commands.spawn((AudioPlayer(sound.clone()), PlaybackSettings::DESPAWN));
        }
    }
}
