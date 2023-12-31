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
        .add_plugins(
            DefaultPlugins
                .set(setup_win)
                .set(ImagePlugin::default_nearest()),
        )
        // startup
        .add_systems(Startup, window_cam::make_camera)
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                spawner_spawn,
                move_player,
                scroll_ground,
                scroll_items,
                collide,
                update_glider_text,
                midair_player,
                animate_player,
            )
                .chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    summon_player!(commands, asset_server, texture_atlases);

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            ..default()
        },
        Spawner {
            spawn_time: Timer::from_seconds(1.0, TimerMode::Repeating),
        },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec3::new(4.5, 4.5, 1.0),
                ..default()
            },
            texture: asset_server.load("snow.png"),
            ..default()
        },
        global::Ground::new(200.0),
    ));

    let text = "i am the textman\nmy text is delicous";

    commands.spawn((
        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 25.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Left)
        .with_style(Style {
            position_type: PositionType::Relative,
            bottom: Val::Px(0.0),
            right: Val::Px(0.0),
            ..default()
        }),
        GliderText,
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
                        player.midair = true;
                        if player.gliders > 0 && !player.has_glider {
                            player.gliders -= 1;
                            player.has_glider = true;
                        }
                    }
                    "glider" => {
                        player.gliders += 1;
                        commands.entity(entity).despawn();
                    }
                    _ => (),
                }
            }
        }
    }
}
