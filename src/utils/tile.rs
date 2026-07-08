use bevy::prelude::*;

pub fn rect_tile_pos_iter(transform: Transform, tile_size: Vec2) -> impl Iterator<Item = Vec3> {
    let area_size = transform.scale.truncate();

    let cols = (area_size.x / tile_size.x) as u32;
    let rows = (area_size.y / tile_size.y) as u32;

    let total_width = cols as f32 * tile_size.x;
    let total_height = rows as f32 * tile_size.y;

    let start_x = -total_width / 2.0 + tile_size.x / 2.0;
    let start_y = -total_height / 2.0 + tile_size.y / 2.0;

    let placement_transform = Transform {
        translation: transform.translation,
        rotation: transform.rotation,
        scale: Vec3::splat(1.0),
    };

    (0..cols).flat_map(move |x| {
        (0..rows).map(move |y| {
            let tile_pos = Vec2::new(
                start_x + x as f32 * tile_size.x,
                start_y + y as f32 * tile_size.y,
            );
            placement_transform.transform_point(tile_pos.extend(0.0))
        })
    })
}

pub fn upper_tile_pos_iter(transform: Transform, tile_size: Vec2) -> impl Iterator<Item = Vec3> {
    let area_size = transform.scale.truncate();

    let cols = (area_size.x / tile_size.x) as u32;
    let rows = (area_size.y / tile_size.y) as u32;

    let total_width = cols as f32 * tile_size.x;
    let total_height = rows as f32 * tile_size.y;

    let start_x = -total_width / 2.0 + tile_size.x / 2.0;
    let start_y = -total_height / 2.0 + tile_size.y / 2.0;

    let placement_transform = Transform {
        translation: transform.translation,
        rotation: transform.rotation,
        scale: Vec3::splat(1.0),
    };

    (0..cols).map(move |x| {
        let tile_pos = Vec2::new(
            start_x + x as f32 * tile_size.x,
            start_y + (rows - 1) as f32 * tile_size.y,
        );
        placement_transform.transform_point(tile_pos.extend(0.0))
    })
}
