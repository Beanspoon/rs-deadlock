use bevy::{color::palettes::css::WHITE_SMOKE, math::VectorSpace, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, draw_grid)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/ARLRDBD.TTF");

    commands.spawn(Camera2dBundle::default());

    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello",
            TextStyle {
                font: font.clone(),
                font_size: 60.,
                ..default()
            },
        ),
        ..default()
    });
}

fn draw_grid(mut gizmos: Gizmos) {
    for index in 0..36 {
        let iso_x = (index % 6) as f32 * 100.0;
        let iso_y = (index / 6) as f32 * 100.0;

        let car_x = (iso_x - iso_y) / 1.5;
        let car_y = iso_x / 3.0 + iso_y / 1.5;
        gizmos.primitive_2d(
            &Rhombus {
                half_diagonals: Vec2 { x: 100.0, y: 75.0 },
            },
            Vec2 { x: car_x, y: car_y },
            0.0,
            WHITE_SMOKE,
        );
    }
}
