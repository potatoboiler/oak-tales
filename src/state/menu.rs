use bevy::prelude::*;

mod button;

use super::AppState;
use button::*;

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>()
            .insert_resource(ClearColor(Color::rgb(1.0, 0.0, 1.0)))
        .add_system_set( SystemSet::on_enter(AppState::MainMenu))//.with_system(construct_initial_buttons))
        // .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(button_system))
        // .add_system_set(SystemSet::on_exit(AppState::MainMenu));
        ;
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

#[derive(Component)]
pub struct ButtonFuncs {
    target: MenuSubActions,
}

fn construct_initial_buttons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(SpriteBundle {
        // texture: materials.add(Color::rgb(0.5, 0.5, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(12.0, 5.0)),
            color: Color::rgb(0.5, 0.5, 0.0),
            ..Default::default()
        },
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

    let mut make_button = |position: Transform, message: String, action: MenuSubActions| {
        commands
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                // texture: button_materials.normal.clone(),
                transform: position,
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text::with_section(
                        message,
                        TextStyle {
                            font: asset_server.load("fonts/cmunss.otf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                        Default::default(),
                    ),
                    ..Default::default()
                });
                parent.spawn().insert(ButtonFuncs { target: action });
            });
    };

    // change button?
    make_button(
        Transform::from_xyz(100.0, 200.0, 0.0),
        "Charsel".to_string(),
        MenuSubActions::CharSelect,
    );

    make_button(
        Transform::from_xyz(10.0, 150.0, 0.0),
        "Charcreate".to_string(),
        MenuSubActions::CharCreate,
    );

    make_button(
        Transform::from_xyz(10.0, 150.0, 0.0),
        "Back".to_string(),
        MenuSubActions::Back,
    );

    make_button(
        Transform::from_xyz(0.0, 150.0, 0.0),
        "Rest".to_string(),
        MenuSubActions::Exit,
    );

    info!("constructed main menu!");
}
