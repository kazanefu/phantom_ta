use bevy::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
mod ground_spawn;

pub struct RoomItemsPlugin;

impl Plugin for RoomItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ground_spawn::GroundSpawnPlugin);
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum ItemKind {
    Ground,
    OneWay,
    Water,
    DamageGround,
    Text(String),
}

#[derive(Clone, Debug)]
pub struct RoomItem {
    pub kind: ItemKind,
    pub transform: Transform,
}

#[derive(Serialize, Deserialize)]
struct ItemSerde {
    kind: ItemKind,
    transform: ItemTransform,
}

impl Serialize for RoomItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let data = ItemSerde {
            kind: self.kind.clone(),
            transform: ItemTransform::from(&self.transform),
        };

        data.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RoomItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let data = ItemSerde::deserialize(deserializer)?;

        Ok(RoomItem {
            kind: data.kind,
            transform: data.transform.into(),
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct ItemTransform {
    // translation x, y, rotation r, scale s_x, s_y
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub r: f32,
    #[serde(default = "default_scale")]
    pub s_x: f32,
    #[serde(default = "default_scale")]
    pub s_y: f32,
}

fn default_scale() -> f32 {
    1.0
}

impl Into<Transform> for ItemTransform {
    fn into(self) -> Transform {
        Transform::from_translation(Vec3::new(self.x, self.y, 0.0))
            .with_rotation(Quat::from_rotation_z(self.r))
            .with_scale(Vec3::new(self.s_x, self.s_y, 1.0))
    }
}

impl From<&Transform> for ItemTransform {
    fn from(value: &Transform) -> Self {
        let (_, _, r) = value.rotation.to_euler(EulerRot::XYZ);

        Self {
            x: value.translation.x,
            y: value.translation.y,
            r,
            s_x: value.scale.x,
            s_y: value.scale.y,
        }
    }
}
