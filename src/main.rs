#![allow(unused_imports)]
use bevy::ecs::event::event_update_condition;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::view::visibility;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle, time::Stopwatch};
use bevy_inspector_egui::egui::Key;
use bevy_kira_audio::{Audio, AudioPlugin, AudioControl};
use bevy_kira_audio::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod global;
use global::global::{GameComponent, Ground};

mod player;
use player::*;

mod floor_items;
use floor_items::*;

mod states;
use states::*;

mod window_cam;

use rand::Rng;

const DEFAULT_SCROLL_SPEED: f32 = 600.0;

fn main() {
    let setup_win = window_cam::setup_window();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(setup_win)
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(AudioPlugin)
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::D)))
        .add_state::<GameState>()
        // always running
        .add_systems(Startup, (window_cam::make_camera, setup_game).chain())
        .add_systems(Update, music_player_update)
        // in-game systems
        .add_systems(
            OnEnter(GameState::InGame), 
            (
                show_or_hide_game!(Visible),
                remove_flooritems,
                reset_player,
            )

        )
        .add_systems(OnExit(GameState::InGame), show_or_hide_game!(Hidden))
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
                .chain()
                .run_if(in_state(GameState::InGame))
        )
        // game over systems
        .add_plugins(states::GameOverPlugin)
        .run();
}

fn setup_game(
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
        GameComponent,
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
        Ground::new(DEFAULT_SCROLL_SPEED),
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
        GameComponent,
    ));

    commands.spawn((NodeBundle {
        ..default()
    }, MusicPlayer{
        start_music: true,
        state: 1,
    }));
}

const MAX_Y_POS_GROUND: f32 = 80.0;

fn scroll_ground(mut ground_query: Query<(&mut Ground, &mut Transform)>, time: Res<Time>) {
    let (ground, mut transform) = ground_query.single_mut();

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
    }
}

fn collide(
    mut commands: Commands,
    mut asset_server: Res<AssetServer>,
    mut player_query: Query<(&mut Player, &Transform)>,
    mut entity_query: Query<(Entity, &Transform, Option<&FloorItem>)>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut audio: Res<bevy_kira_audio::Audio>,
    mut query: Query<Entity, With<GameComponent>>,
    mut music_query: Query<&mut MusicPlayer>
) {
    let (mut player, player_transform) = player_query.single_mut();
    let player_size = player_transform.scale.truncate();
    let mut music = music_query.single_mut();

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
                        if !player.midair {
                            player.gliders += 1;
                            commands.entity(entity).despawn();
                        }
                    }
                    "extra life" => {
                        if !player.midair {
                            player.lives += 1;
                            commands.entity(entity).despawn();
                        }
                    }
                    "roadblock" => {
                        if !player.midair {
                            player.lives -= 1;
                            if player.lives < 0 {
                                audio.stop();
                                audio.play(asset_server.load("deathtune.wav"));
                                next_state.set(GameState::GameOver);
                                music.state = 0;
                                music.start_music = true;
                            }
                            else {
                                audio.play(asset_server.load("hittable.mp3"));
                            }
                            commands.entity(entity).despawn();
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
