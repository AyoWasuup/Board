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

use rand::Rng;

const DEFAULT_SCROLL_SPEED: f32 = 1.0;

fn main() {
    let setup_win = window_cam::setup_window();

    App::new()
        .add_plugins(DefaultPlugins.set(setup_win))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_systems(Startup, window_cam::make_camera)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, move_player)
        .add_systems(FixedUpdate, scroll_ground)
        .add_systems(FixedUpdate, scroll_items)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    summon_player!(commands, Vec3::new(0.0, 30.0, 0.0));
    make_flooritem_base!(commands);

    commands
        .spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, -1.0),
                    scale: Vec3::new(5.0, 5.0, 1.0),
                    ..default()
                },
                texture: asset_server.load("snow.png"),
                ..default()
            },
            Ground::new(50.0),
        ))
        .insert(Collider::cuboid(GROUND_WIDTH, 400.0, 0.0));
}

const MAX_Y_POS_GROUND: f32 = 80.0;

fn scroll_ground(mut ground_query: Query<(&mut Ground, &mut Transform)>, time: Res<Time>) {
    let (mut ground, mut transform) = ground_query.single_mut();

    if MAX_Y_POS_GROUND > transform.translation.y {
        transform.translation.y += ground.scroll_speed * time.delta_seconds();
    } else {
        transform.translation.y = 0.0;
    }
}

fn scroll_items(
    mut scroll_query: Query<(&mut FloorItem, &mut Transform)>,
    ground: Query<&Ground>,
    time: Res<Time>,
) {
    for (mut flooritem, mut transform) in &mut scroll_query {
        transform.translation.y += ground.single().scroll_speed * time.delta_seconds();
        println!("flooritem: {}", transform.translation);
    }
}
