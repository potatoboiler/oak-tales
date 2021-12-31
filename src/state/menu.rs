use bevy::prelude::*;
use bevy::ui::widget;

use super::AppState;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::MainMenu).with_system(construct_initial_buttons.system()),
        );
    }
}

fn construct_initial_buttons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(12.0, 5.0)),
        ..Default::default()
    });

    commands.spawn_bundle(bevy::text::Text2dBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "test_str".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/cmunss.otf"),
                        font_size: 20.0,
                        color: Color::BLUE,
                        ..Default::default()
                    },
                },
                TextSection {
                    value: "asdf".to_string(),
                    style: TextStyle {
                        font: asset_server.load("fonts/cmunss.otf"),
                        font_size: 20.0,
                        color: Color::BLUE,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, -50.0, 2.0),
        ..Default::default()
    });

    info!("constructed main menu!");
}
