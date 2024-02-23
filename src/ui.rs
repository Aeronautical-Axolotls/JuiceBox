use bevy_egui::{egui,EguiContexts};
use crate::file_system::{self, load_scene, save_scene};

use bevy::{prelude, ecs::system::Commands};
use bevy::asset::AssetServer;
use bevy::ecs::system::Res;
use bevy::ecs::world::{self, World};

pub fn ui_base(mut contexts: EguiContexts,
               mut commands: Commands,
               asset_server: Res<AssetServer>) {
    egui::Window::new("Dev Tools").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
        if ui.button("Load test_load_2.scn.ron WIP").clicked() {
            load_scene(commands, asset_server);
        }
        if ui.button("Save into test_save_1.scn.ron WIP -NOT WORKING AT ALL-").clicked() {
            //save_scene(world);
            println!("Save Button Pressed!");
        }
        if ui.button("Open File Explorer Test").clicked() {
            println!("{}", file_system::get_file());
        }
    });
}