/*
References:
https://www.youtube.com/watch?v=qbeu0Mw1HLY
https://github.com/mwbryant/rpg-bevy-tutorial/ refer to branches for specific tutorials. buttons are at tutorial 6
*/
//! This example illustrates the various features of Bevy UI.

pub mod game_states;

use bevy::{
    prelude::{default, App, AssetServer, Camera2dBundle, Commands, Res},
    sprite::SpriteBundle,
    DefaultPlugins,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_system(ui_example_system)
        .add_startup_system(start_game)
        .run();
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn start_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("img/Untitled.png"),
        ..default()
    });
}
