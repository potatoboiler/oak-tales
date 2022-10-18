/*
References:
https://www.youtube.com/watch?v=qbeu0Mw1HLY
https://github.com/mwbryant/rpg-bevy-tutorial/ refer to branches for specific tutorials. buttons are at tutorial 6 
*/
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

mod game_states;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    MainMenu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .add_state(AppState::MainMenu)
        .add_startup_system(play_music)
        .add_startup_system(spawn_camera)
        .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(menu_setup))
        .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(menu_despawn))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component)]
struct MenuItem;

fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn menu items here
    // todo!("asdasd")
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),
                // margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MenuItem);
}

fn menu_despawn(mut commands: Commands, query: Query<Entity, With<MenuItem>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
    // todo!()
}

fn play_music(app_state: Res<State<AppState>>, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("sounds/NightMarket.ogg");
    match app_state.current() {
        // _ => todo!(),
        _ => audio.play(music),
    };
}

fn load_character() {
    todo!()
}
