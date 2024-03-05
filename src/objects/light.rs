use bevy::prelude::*;

#[derive(Debug)]
pub struct DirectionalLightPlugin;

impl Plugin for DirectionalLightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_light);
    }
}

fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}
