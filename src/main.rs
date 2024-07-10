use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    let rhombus = Rhombus::new(100., 100.);
    let rhombus_handle = Mesh2dHandle(meshes.add(rhombus));
    commands.spawn(MaterialMesh2dBundle {
        mesh: rhombus_handle,
        material: materials.add(Color::hsl(0.0, 0.3, 1.)),
        ..default()
    });
}
