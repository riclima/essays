use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup_field, setup).chain())
        .add_systems(FixedUpdate, move_paddle)
        .run();
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Player {
    id: u8,
}

// #[derive(Component)]
// struct Score {
//     value: usize,
// }

#[derive(Resource)]
struct Field {
    dimensions: Vec2,
    scale: f32,
    wall_thickness: f32,
}

impl Field {
    fn new(window_width: f32, window_height: f32) -> Self {
        const COURT_SIZE: Vec2 = Vec2::new(2.74, 1.525);
        let wall_thickness: f32 = 10.0;

        let scale = (window_width / COURT_SIZE.x)
            .min((window_height - (2.0 * wall_thickness)) / COURT_SIZE.y);
        let dimensions = Vec2::new(COURT_SIZE.x * scale, COURT_SIZE.y * scale);

        Field {
            dimensions,
            scale,
            wall_thickness,
        }
    }
}

fn setup_field(mut commands: Commands, window: Single<&Window>) {
    let field = Field::new(window.width(), window.height());
    commands.insert_resource(field);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    // asset_server: Res<AssetServer>,
    field: Res<Field>,
) {
    // Camera
    commands.spawn(Camera2d);

    // TODO: load the ball colision sounds here
    // Sound
    // load_sounds(&mut commands, &asset_server);

    // Net
    let num_units = 63;
    let num_dashes = 16;
    let dash_unit = field.dimensions.y / (num_units as f32);
    let dash_spacing = dash_unit;
    let dash_size = dash_unit * 3.0;

    for i in 0..num_dashes {
        let y_position =
            (i as f32 * (dash_size + dash_spacing)) - (field.dimensions.y / 2.0) + dash_size / 2.0;
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::default())),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform {
                translation: Vec3::new(0.0, y_position, 0.0),
                scale: Vec3::new(field.wall_thickness / 3.0, dash_size, 0.0),
                ..Default::default()
            },
        ));
    }

    // Walls
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform {
            translation: Vec3::new(
                0.0,
                (field.dimensions.y / 2.0) + (field.wall_thickness / 2.0),
                0.0,
            ),
            scale: Vec3::new(field.dimensions.x, field.wall_thickness, 1.0),
            ..Default::default()
        },
    ));
    commands.spawn((
        Sprite::from_color(Color::WHITE, Vec2::ONE),
        Transform {
            translation: Vec3::new(
                0.0,
                -(field.dimensions.y / 2.0) - (field.wall_thickness / 2.0),
                0.0,
            ),
            scale: Vec3::new(field.dimensions.x, field.wall_thickness, 1.0),
            ..Default::default()
        },
    ));

    // Paddles
    // These need to be entitites
    let paddle_dimensions: Vec2 = Vec2::new(0.02 * field.scale, 0.20 * field.scale);
    let paddle_colour: Color = Color::WHITE;

    // Player 1
    let paddle1 = (
        Sprite::from_color(paddle_colour, Vec2::ONE),
        Transform {
            translation: Vec3::new(-(field.dimensions.x - paddle_dimensions.x) / 2.0, 0.0, 0.0),
            scale: paddle_dimensions.extend(1.0),
            ..Default::default()
        },
        Player { id: 0 },
    );
    commands.spawn(paddle1);

    // Player 2
    let paddle2 = (
        Sprite::from_color(paddle_colour, Vec2::ONE),
        Transform {
            translation: Vec3::new((field.dimensions.x - paddle_dimensions.x) / 2.0, 0.0, 0.0),
            scale: paddle_dimensions.extend(1.0),
            ..Default::default()
        },
        Player { id: 1 },
    );
    commands.spawn(paddle2);

    // TODO Scoreboard

    // Play ball!
    let ball_diameter = Vec2::splat(0.03 * field.scale).extend(1.0);
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(paddle_colour)),
        Transform::from_scale(ball_diameter),
        Ball,
    ));
}

fn move_paddle(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
    field: Res<Field>,
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
        let half_height = field.dimensions.y / 2.0;
        transform.translation.y = new_y.clamp(-half_height, half_height);
    }
}
