use bevy::{color::palettes::css::BLACK, image::BevyDefault, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource)]
struct Field(Vec2);

impl Field {
    fn new(scale: f32) -> Self {
        const COURT_SIZE: Vec2 = Vec2::new(2.74, 1.525);
        const WALL_WIDTH: f32 = 0.1;
        Field(COURT_SIZE + Vec2::splat(WALL_WIDTH))
    }
}

fn setup(mut commands: Commands) {
    // Firstly, we create the resources that we will use in our game.
    let background = ClearColor(BLACK.into());

    // Set up the game's camera.
    commands.spawn(Camera2d);

    // Now that we are done with resources, we can insert them into the
    // world so other systems can use them.
}
