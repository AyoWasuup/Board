use bevy::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier3d::prelude::*;

mod global;
use global::*;

mod player;
use player::*;

mod floor_items;
use floor_items::*;

mod window_cam;

fn main() {
    let setup_win = window_cam::setup_window();

    App::new()
        .add_plugins(DefaultPlugins.set(setup_win))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, window_cam::make_camera)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_player)
        .add_systems(FixedUpdate, scroll_ground)
        .run();
}

fn setup(mut commands: Commands) {
    summon_player!(commands, Vec3::new(0.0, 30.0, 0.0));

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    scale: Vec3::new(GROUND_WIDTH, 400.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::WHITE,
                    ..default()
                },
                ..default()
            },
            Ground::new(50.0),
            Scroll,
        ))
        .insert(Collider::cuboid(GROUND_WIDTH, 400.0, 0.0));
}

const MAX_Y_POS_GROUND: f32 = 100.0;

fn scroll_ground(mut ground_query: Query<(&mut Ground, &mut Transform)>, time: Res<Time>) {
    let mut ground_speed = 0.0;
    for (mut ground, mut transform) in &mut ground_query {
        if MAX_Y_POS_GROUND > transform.translation.y {
            transform.translation.y += ground.scroll_speed * time.delta_seconds();
        } else {
            transform.translation.y = 0.0;
        }
        ground_speed = ground.scroll_speed;
        println!("{}, {}", transform.translation.y, MAX_Y_POS_GROUND);
    }
}
