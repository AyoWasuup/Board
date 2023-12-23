#![allow(dead_code)]
use bevy::prelude::*;
use rand::Rng;

const PICKUP_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);

#[path = "global.rs"]
mod global;

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
                    scale: Vec3::new(30.0, 30.0, 1.0),
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
        //.with_children(|children| {
        //    children
        //        .spawn(Collider::cuboid(30.0, 30.0, 1.0))
        //        .insert(Sensor);
        //});
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
                    scale: Vec3::new(30.0, 30.0, 1.0),
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
        //.with_children(|children| {
        //    children
        //        .spawn(Collider::cuboid(30.0, 30.0, 1.0))
        //        .insert(Sensor)
        //        .insert(
        //            ActiveCollisionTypes::default() | ActiveCollisionTypes::COLLISION_EVENTS,
        //        );
        //});
    };
}
