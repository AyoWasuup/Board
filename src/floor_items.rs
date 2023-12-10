use bevy::prelude::*;
use rand::Rng;

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

    pub fn get_type(&mut self) -> &str {
        &self.item_type
    }
}

#[macro_export]
macro_rules! make_flooritem_base {
    ($commands:ident) => {
        let mut rng = rand::thread_rng();
        let xpos: f32 = rng.gen_range(-400.0..400.0);
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
        ));
    };
}
