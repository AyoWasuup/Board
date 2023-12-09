use bevy::prelude::*;

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
    ($commands:ident, $vec:expr, $scroll:expr) => {
        $commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: $vec,
                    scale: Vec3::new(30.0, 30.0, 30.0),
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
