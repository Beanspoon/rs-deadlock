use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
