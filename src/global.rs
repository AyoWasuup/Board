use bevy::{core_pipeline::core_3d::graph::node::DEFERRED_PREPASS, prelude::*};

pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const GROUND_WIDTH: f32 = 490.0;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Ground {
    pub scroll: bool,
    pub scroll_speed: f32,
}

impl Ground {
    pub fn new(speed: f32) -> Ground {
        Ground {
            scroll: (if speed == 0.0 { false } else { true }),
            scroll_speed: speed,
        }
    }
}
