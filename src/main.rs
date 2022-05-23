use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![TextSection { value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas auctor, nunc ac faucibus fringilla.".to_string(), style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,font: asset_server.load("fonts/monogram.ttf")
                } }],
                alignment: Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                max_size: Size::new(Val::Percent(50.), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        });
}
