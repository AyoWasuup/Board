use bevy::winit::WinitWindows;
use bevy::{
    prelude::*,
    render::view::WindowRenderPlugin,
    window::{WindowDestroyed, WindowResized, WindowResolution},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn make_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    //});
}

pub fn setup_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Board?".into(),
            resizable: false,
            resolution: WindowResolution::new(480.0, 800.0).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    }
}
