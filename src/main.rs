use bevy::prelude::*;

// let transform = Mat2::IDENTITY
//     .mul_mat2(&Mat2::from_scale_angle(Vec2 { x: 0.63, y: 1.0 }, PI / 4.0))
//     .inverse();
const ISO_TRANSFORM: Mat2 =
    Mat2::from_cols_array_2d(&[[1.1223918, -0.7071068], [1.1223918, 0.7071068]]);

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
        let colour = Color::hsl(360.0 * index as f32 / 36.0, 0.95, 0.7);
        let car_x = (index % 6) as f32 * 100.0;
        let car_y = (index / 6) as f32 * 100.0;

        let iso_coords: Vec2 = ISO_TRANSFORM.mul_vec2(Vec2 { x: car_x, y: car_y });
        gizmos.primitive_2d(
            &Rhombus {
                half_diagonals: Vec2 { x: 100.0, y: 63.0 },
            },
            iso_coords,
            0.0,
            colour,
        );
    }
}
