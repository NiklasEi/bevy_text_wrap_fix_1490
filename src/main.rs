use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(UiCameraBundle::default())
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection { value: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas auctor, nunc ac faucibus fringilla.".to_string(), style: TextStyle {
                    font_size: 60.0,
                    color: Color::WHITE,font: asset_server.load("fonts/monogram.ttf")
                } }],
                alignment: Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..Default::default()
                },
                max_size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        });
}
