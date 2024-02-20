use bevy::{input::mouse::MouseMotion, prelude::*, render::render_resource::PipelineLayout, time::Stopwatch, ecs::change_detection::MutUntyped};
use std::time::Duration;
use crate::global::global::GameComponent;

const SPEED: f32 = 25.0;
pub const PLAYER_SCALE: Vec3 = Vec3::new(25.0, 50.0, 0.0);
pub const PLAYER_DEFAULT_POS: Vec3 = Vec3::new(0.0, 200.0, 0.0);

#[macro_export]
macro_rules! player_jump_time {
    () => {
        Timer::from_seconds(0.9, TimerMode::Once)
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
    pub lives: i32,
    pub has_glider: bool,
    pub energy: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            midair: false,
            midair_time: (0, 5),
            gravity: 9.0,
            has_glider: false,
            gliders: 0,
            lives: 2,
            energy: false,
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
            rotate = ev.delta.x / 2.0;
        }

        if rotate > 0.5 {
            player_sprite.current = 1;
        } else if rotate < -0.5 {
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
    mut query: Query<(&PlayAnimation, &mut TextureAtlasSprite, &mut MidairTimer), With<Player>>,
    mut qplayer: Query<&mut Player>,
    time: Res<Time>,
) {
    let mut player = qplayer.single_mut();

    for (player_anim, mut player_sprite, mut midair_timer) in &mut query {
        if !player.midair {
            midair_timer.on_ground_for.unpause();
            midair_timer.on_ground_for.tick(time.delta());

            if midair_timer.on_ground_for.elapsed() < Duration::from_secs_f32(0.06) 
                && midair_timer.on_ground_for.elapsed() > Duration::from_secs_f32(0.02) {
                player_sprite.index = 4;
            }
            else {
                player_sprite.index = player_anim.current;
            }
        }
        else {
            player_sprite.index = 3;
            midair_timer.on_ground_for.reset();
            midair_timer.on_ground_for.pause();
        }
    }
}

#[derive(Component)]
pub struct MidairTimer {
    pub time: Timer,
    pub on_ground_for: Stopwatch,
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
                            4.0
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

    text.sections[0].value = {
        "gliders: ".to_string() + player.gliders.to_string().as_str() + "\n" +
        "lives: " + player.lives.to_string().as_str()
    };
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
                visibility: Visibility::Hidden,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: PLAYER_DEFAULT_POS,
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..default()
                },
                ..default()
            },
            Player::new(),
            GameComponent,
            MidairTimer {
                time: player_jump_time!(),
                on_ground_for: Stopwatch::new(),
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

// it does the job
pub fn hide_player(
    mut vis_query: Query<&mut Visibility, With<Player>>,
){ 
    for mut visible in &mut vis_query {
        *visible = Visibility::Hidden;
    }
}

// this also does the job
pub fn show_player(
    mut vis_query: Query<&mut Visibility, With<Player>>,
){ 
    for mut visible in &mut vis_query {
        *visible = Visibility::Visible;
    }    
}

pub fn reset_player(mut pos: Query<&mut Transform, With<Player>>, mut player_query: Query<&mut Player>){
    let mut player = player_query.single_mut();
    player.lives = 2;
    player.gliders = 0;
    player.midair = false;
    player.energy = false;

    for mut transform in &mut pos {
        transform.translation = PLAYER_DEFAULT_POS;
    }
}
