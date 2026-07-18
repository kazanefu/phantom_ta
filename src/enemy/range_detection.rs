use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::player::Player;

pub struct RangeDetectionPlugin;

impl Plugin for RangeDetectionPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Serialize, Deserialize, Clone, Copy)]
pub struct DetectionRange {
    is_inside: bool,
    pub range: RangeShape,
    pub center: Vec2,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum RangeShape {
    Circle(f32),
    Rectangle(f32, f32),
}

impl RangeShape {
    pub fn contains(&self, point: Vec2) -> bool {
        match self {
            RangeShape::Circle(radius) => point.length_squared() <= *radius * *radius,
            RangeShape::Rectangle(width, height) => {
                point.x.abs() <= *width / 2.0 && point.y.abs() <= *height / 2.0
            }
        }
    }
}

impl DetectionRange {
    pub fn contains(&self, point: Vec2) -> bool {
        let relative_point = point - self.center;
        self.range.contains(relative_point)
    }
    pub fn is_inside(&self) -> bool {
        self.is_inside
    }
}

fn check_inside(
    mut detector_que: Query<&mut DetectionRange>,
    player_que: Query<&Transform, With<Player>>,
) {
    for mut detector in &mut detector_que {
        detector.is_inside = player_que
            .iter()
            .any(|transform| detector.contains(transform.translation.xy()))
    }
}
