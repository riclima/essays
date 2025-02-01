use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume},
    prelude::*,
};

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

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Collider)]
struct Wall;

#[derive(Event, Default)]
struct CollisionEvent;

const BALL_RADIUS: f32 = 12.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK.into()))
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (move_paddle, apply_velocity, check_for_collisions).chain(),
        )
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
        Wall,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, offset, 0.0),
            scale: Vec3::new(COURT_WIDTH, WALL_THICKNESS, 0.0),
            ..Default::default()
        },
        Collider,
    ));
    commands.spawn((
        Wall,
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform {
            translation: Vec3::new(0.0, -offset, 0.0),
            scale: Vec3::new(COURT_WIDTH, WALL_THICKNESS, 0.0),
            ..Default::default()
        },
        Collider,
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
        Collider,
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
        Collider,
    ));

    // Set up the game's ball.
    commands.spawn((
        Ball,
        Mesh2d(meshes.add(Circle::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(Color::WHITE)),
        Transform::default(),
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

fn check_for_collisions(
    // mut commands: Commands,
    ball_query: Single<(&mut Velocity, &Transform), With<Ball>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.into_inner();

    for collider_transform in &collider_query {
        let collision = ball_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_RADIUS),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.0,
            ),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut reflect_x = false;
            let mut reflect_y = false;

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
            }

            if reflect_x {
                ball_velocity.x *= -1.0;
            }

            if reflect_y {
                ball_velocity.y *= -1.0;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Collision {
    Left,
    Right,
    Top,
    Bottom,
}

fn ball_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest;
    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x > 0.0 {
            Collision::Right
        } else {
            Collision::Left
        }
    } else {
        if offset.y > 0.0 {
            Collision::Top
        } else {
            Collision::Bottom
        }
    };

    Some(side)
}
