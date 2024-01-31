#![allow(dead_code)]
use crate::{make_glider, make_ramp, make_extra_life, make_roadblock_base};
use bevy::prelude::*;
use rand::Rng;

const PICKUP_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

#[path = "global.rs"]
mod global;

#[derive(Component)]
pub struct Spawner {
    pub spawn_time: Timer,
}

pub fn spawner_spawn(mut query: Query<&mut Spawner>, time: Res<Time>, mut commands: Commands) {
    for mut spawner in &mut query {
        spawner.spawn_time.tick(time.delta());

        if spawner.spawn_time.just_finished() {
            let mut rng = rand::thread_rng();
            let spawn_index = rng.gen_range(0..100);

            if spawn_index <= 15 {
                make_glider!(commands);
            } else if spawn_index <= 60 {
                make_ramp!(commands);
            } else if spawn_index <= 61 {
                make_extra_life!(commands);
            } else if spawn_index <= 80 {
                make_roadblock_base!(commands);
            }
        }
    }
}

#[derive(Component)]
pub struct FloorItem {
    pub item_type: String,
}

impl FloorItem {
    pub fn new(itype: &str) -> FloorItem {
        FloorItem {
            item_type: itype.to_string(),
        }
    }

    pub fn get_type(&self) -> &str {
        &self.item_type
    }
}

#[macro_export]
macro_rules! make_flooritem_base {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(40.0, 30.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("default"),
        ))
    };
}

#[macro_export]
macro_rules! make_roadblock_base {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(60.0, 75.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::GREEN,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("roadblock"),
        ))
    };
}

#[macro_export]
macro_rules! make_ramp {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(60.0, 20.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::GRAY,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("ramp"),
        ))
    };
}

#[macro_export]
macro_rules! make_glider {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(30.0, 30.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::RED,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("glider"),
        ))
    };
}

#[macro_export]
macro_rules! make_extra_life {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(30.0, 30.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLUE,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("extra life"),
        ))
    };
}

#[macro_export]
macro_rules! make_energy {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-200.0..200.0);
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(xpos, -400.0, 1.0),
                    scale: Vec3::new(30.0, 10.0, 1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::GREEN,
                    ..default()
                },
                ..default()
            },
            FloorItem::new("energy"),
        ))
    };
}
