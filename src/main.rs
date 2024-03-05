use behavior::{
    collision_detection::CollisionDetectionPlugin, despawn::DespawnPlugin, movement::MovementPlugin,
};
use bevy::prelude::*;
use game::{asset_loader::AssetLoaderPlugin, schedule::SchedulePlugin, state::StatePlugin};
use objects::{
    asteroids::AsteroidPlugin, camera::CameraPlugin, light::DirectionalLightPlugin,
    spaceship::SpaceShipPlugin,
};

mod behavior;
mod game;
mod objects;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // custom
        .add_plugins(DirectionalLightPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceShipPlugin)
        .add_plugins(AsteroidPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DespawnPlugin)
        // .add_plugins(DebugPlugin)
        .run();
}
