use bevy::{input::mouse::MouseMotion, prelude::*, render::render_resource::PipelineLayout};
#[path = "global.rs"]
mod global;

use global::*;

const SPEED: f32 = 25.0;
pub const PLAYER_SCALE: Vec3 = Vec3::new(25.0, 50.0, 0.0);

#[derive(Component)]
pub struct Player {
    pub midair: bool,
}

impl Player {
    pub fn new() -> Player {
        Player { midair: false }
    }
}

pub fn move_player(
    mut query: Query<(&mut Player, &mut Transform)>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    for (mut player, mut transform) in &mut query {
        let mut movedx = 0.0;
        let mut rotate = 0.0;

        for ev in mouse_motion.read() {
            // println!("Mouse moved x: {} y: {}", ev.delta.x, ev.delta.y);
            movedx = ev.delta.x;
            rotate = ev.delta.x;
        }

        transform.translation.x += (movedx * SPEED) * time.delta_seconds();
        // both of the ground's and player's widths are halfed because in bevy the position is in the centre of
        // the renderd object
        let clamped_transform = transform.translation.x.clamp(-200.0, 200.0);

        transform.translation.x = clamped_transform;

        transform.rotate_z(rotate * time.delta_seconds());
        let clamped_rotation = transform.rotation.z.clamp(-25.5, 90.0);
        transform.rotation = Quat::from_rotation_z(clamped_rotation);
    }
}

#[macro_export]
macro_rules! summon_player {
    ($commands:ident, $vec:expr) => {
        $commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: $vec,
                        scale: PLAYER_SCALE,
                        ..default()
                    },
                    sprite: Sprite {
                        color: Color::BLUE,
                        ..default()
                    },
                    ..default()
                },
                Player::new(),
            ))
            .with_children(|children| {
                //children
                //    //.spawn(Collider::cuboid(
                //        PLAYER_SCALE.x,
                //        PLAYER_SCALE.y,
                //        PLAYER_SCALE.z,
                //    ))
                //    .insert(Sensor);
            });
    };
}
