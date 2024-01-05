use bevy::{input::mouse::MouseMotion, prelude::*, render::render_resource::PipelineLayout};
use std::time::Duration;
#[path = "global.rs"]
mod global;

use global::*;

const SPEED: f32 = 25.0;
pub const PLAYER_SCALE: Vec3 = Vec3::new(25.0, 50.0, 0.0);
pub const PLAYER_DEFAULT_POS: Vec3 = Vec3::new(0.0, 200.0, 0.0);

#[macro_export]
macro_rules! player_jump_time {
    () => {
        Timer::from_seconds(0.7, TimerMode::Once)
    };
}

#[derive(Component)]
pub struct PlayAnimation {
    pub first: usize,
    pub last: usize,
    pub current: usize,
}

#[derive(Component)]
pub struct Player {
    pub midair: bool,
    pub midair_time: (usize, usize),
    pub gravity: f32,
    pub gliders: i32,
    pub has_glider: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            midair: false,
            midair_time: (0, 5),
            gravity: 15.0,
            has_glider: false,
            gliders: 0,
        }
    }

    pub fn hit_by_ramp(&mut self) {
        self.midair = true;
    }
}

pub fn move_player(
    mut query: Query<(&mut Player, &mut Transform)>,
    mut player_anim: Query<&mut PlayAnimation, With<Player>>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    let mut player_sprite = player_anim.single_mut();

    for (mut player, mut transform) in &mut query {
        let mut movedx = 0.0;
        let mut rotate = 0.0;

        for ev in mouse_motion.read() {
            movedx = ev.delta.x;
            rotate = ev.delta.x;
        }

        if rotate > 1.0 {
            player_sprite.current = 1;
        } else if rotate < -1.0 {
            player_sprite.current = 2;
        } else {
            player_sprite.current = 0;
        }

        transform.translation.x += (movedx * SPEED) * time.delta_seconds();
        // both of the ground's and player's widths are halfed because in bevy the position is in the centre of
        // the renderd object
        let clamped_transform = transform.translation.x.clamp(-200.0, 200.0);

        transform.translation.x = clamped_transform;

        transform.rotate_z(rotate * time.delta_seconds());
        let clamped_rotation = transform.rotation.z.clamp(-15.0, 79.5);
        transform.rotation = Quat::from_rotation_z(clamped_rotation);
    }
}

pub fn animate_player(
    mut query: Query<(&mut PlayAnimation, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut player, mut player_sprite) in &mut query {
        player_sprite.index = player.current;
    }
}

#[derive(Component)]
pub struct MidairTimer {
    pub time: Timer,
    pub repeated: i32,
}

pub fn midair_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Player, &mut Transform, &mut MidairTimer)>,
) {
    let (mut player, mut player_transform, mut midair_timer) = player_query.single_mut();

    let jumptime_y = 20.0;
    let jumptime_z = 3.5;

    if player.midair {
        midair_timer.time.tick(time.delta());
        if midair_timer.time.finished() {
            midair_timer.repeated += 1;

            if player_transform.translation.y > PLAYER_DEFAULT_POS.y {
                player_transform.translation.y -=
                    player.gravity * (midair_timer.repeated / 5) as f32 / {
                        if player.has_glider {
                            2.0
                        } else {
                            1.0
                        }
                    } * time.delta_seconds();
            } else {
                player_transform.translation.y = PLAYER_DEFAULT_POS.y;
                midair_timer.time.reset();
                midair_timer
                    .time
                    .set_duration(player_jump_time!().duration());
                player.has_glider = false;
                player.midair = false;
            }
        } else if player_transform.translation.y == PLAYER_DEFAULT_POS.y {
            player_transform.translation.y += jumptime_y;
            player_transform.translation.z += jumptime_z;
        }
    }
}

#[derive(Component)]
pub struct GliderText;

pub fn update_glider_text(
    mut player_query: Query<&mut Player>,
    mut text_query: Query<&mut Text, With<GliderText>>,
) {
    let mut player = player_query.single_mut();
    let mut text = text_query.single_mut();

    text.sections[0].value = "gliders: ".to_string() + player.gliders.to_string().as_str();
}

#[macro_export]
macro_rules! summon_player {
    ($commands:ident, $asset_server:ident, $texture_atlases:ident) => {
        let texture_handle = $asset_server.load("player.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 7, 1, None, None);
        let texture_atlas_handle = $texture_atlases.add(texture_atlas);

        $commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: PLAYER_DEFAULT_POS,
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Player::new(),
            MidairTimer {
                time: player_jump_time!(),
                repeated: 0,
            },
            PlayAnimation {
                first: 0,
                last: 2,
                current: 0,
            },
        ))
    };
}
