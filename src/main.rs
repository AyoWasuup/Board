use bevy::ecs::event::event_update_condition;
use bevy::prelude::*;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

mod global;

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
        // startup
        .add_systems(Startup, window_cam::make_camera)
        .add_systems(Startup, setup)
        // fixedupdate
        //.add_systems(FixedUpdate, move_player)
        //.add_systems(FixedUpdate, scroll_ground)
        //.add_systems(FixedUpdate, scroll_items)
        //.add_systems(FixedUpdate, collide)
        .add_systems(
            FixedUpdate,
            (
                move_player,
                scroll_ground,
                scroll_items,
                collide,
                midair_player,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    summon_player!(commands);
    make_ramp!(commands);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::new(5.0, 5.0, 1.0),
                ..default()
            },
            texture: asset_server.load("snow.png"),
            ..default()
        },
        global::Ground::new(200.0),
    ));
}

const MAX_Y_POS_GROUND: f32 = 80.0;

fn scroll_ground(mut ground_query: Query<(&mut global::Ground, &mut Transform)>, time: Res<Time>) {
    let (mut ground, mut transform) = ground_query.single_mut();

    if MAX_Y_POS_GROUND > transform.translation.y {
        transform.translation.y += ground.scroll_speed * time.delta_seconds();
    } else {
        transform.translation.y = 0.0;
    }
}

fn scroll_items(
    mut scroll_query: Query<(&mut FloorItem, &mut Transform)>,
    ground: Query<&global::Ground>,
    time: Res<Time>,
) {
    for (mut flooritem, mut transform) in &mut scroll_query {
        transform.translation.y += ground.single().scroll_speed * time.delta_seconds();
    }
}

fn collide(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut entity_query: Query<(Entity, &Transform, Option<&FloorItem>)>,
) {
    let (mut player, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();

    for (entity, transform, maybe_flooritem) in &mut entity_query {
        let collision = bevy::sprite::collide_aabb::collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            if maybe_flooritem.is_some() {
                let flooritem = maybe_flooritem.unwrap();

                match flooritem.get_type() {
                    "ramp" => {
                        if !player.midair {
                            player.midair = true;
                        }
                    }
                    _ => (),
                }

                // despawns the pickup
                //commands.entity(entity).despawn();
            }
        }
    }
}
