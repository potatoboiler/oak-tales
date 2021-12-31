use bevy::prelude::*;

mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
}

pub struct StateHandler;
// need a stack here
impl Plugin for StateHandler {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(init_cameras.system())
            .add_state(AppState::MainMenu)
            .add_plugin(menu::Menu);
    }
}

struct Character;
fn enter_game(mut app_state: ResMut<State<AppState>>, mut character: Character) {
    app_state.set(AppState::InGame).unwrap();
}

fn init_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
