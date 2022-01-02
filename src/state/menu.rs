use bevy::prelude::*;

mod button;

use super::AppState;
use button::*;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 1.0)))
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(construct_initial_buttons.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::MainMenu).with_system(button_system.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu));
    }
}

// https://stackoverflow.com/questions/64420915/multiple-buttons-without-multiple-button-systems
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuSubActions {
    CharSelect,
    CharCreate,
    Exit,
    Back,
}

struct ButtonFuncs {
    target: MenuSubActions,
}

fn construct_initial_buttons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.5, 0.5, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite::new(Vec2::new(12.0, 5.0)),
        ..Default::default()
    });

    // text box
    commands.spawn_bundle(bevy::text::Text2dBundle {
        text: Text {
            sections: vec![TextSection {
                value: "MapleS2ory".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/cmunss.otf"),
                    font_size: 100.0,
                    color: Color::ORANGE,
                    ..Default::default()
                },
            }],
            alignment: TextAlignment {
                vertical: VerticalAlign::Top,
                horizontal: HorizontalAlign::Left,
                ..Default::default()
            },
            ..Default::default()
        },
        transform: Transform::from_xyz(-210.0, 270.0, 2.0),
        ..Default::default()
    });

    // change button?
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/cmunss.otf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(10.0), Val::Px(6.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            transform: Transform::from_xyz(100.0, 150.0, 75.0),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    "Button",
                    TextStyle {
                        font: asset_server.load("fonts/cmunss.otf"),
                        font_size: 10.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

    info!("constructed main menu!");
}
