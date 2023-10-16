use std::ops::Deref;

use bevy::prelude::*;
mod apps;
pub mod bevy_plugin_shader2d;

pub fn examples(example: String) {
    match example.deref() {
        "single-sdf-distance-as-gradient" => {
            apps::single_sdf_distance_as_gradient::app();
        }
        "single-sdf-distance-as-gradient-with-mouse" => {
            apps::single_sdf_distance_as_gradient_with_mouse::app();
        }
        "single-sdf-distance-as-gradient-with-mouse-and-inner-ray" => {
            apps::single_sdf_distance_as_gradient_with_mouse_and_inner_ray::app();
}
        "single-sdf-distance-as-gradient-with-abs-mouse" => {
            apps::single_sdf_distance_as_gradient_with_abs_mouse::app();
}
        "single-sdf-distance-as-gradient-with-algorithm" => {
            apps::single_sdf_distance_as_gradient_with_algorithm::app();
}
        "single-sdf-distance-as-circle" => {
            apps::single_sdf_distance_as_circle::app();
}
        "single-sdf-distance-as-border" => {
            apps::single_sdf_distance_as_border::app();
}
        _ => {
            panic!("example doesn't exist");
        }
    }
}

pub fn center_sdf(uv: Vec2, width_height: Vec2) -> Vec2 {
    let viewport_width = width_height.x;
    let viewport_height = width_height.y;
    // one side is probably going to be shorter than
    // the other
    let min_viewport_size =
        viewport_width.min(viewport_height);
    let max_viewport_size =
        viewport_width.max(viewport_height);

    // the adjustment by which we need to reposition
    // the longer side for the shader result to
    // still be centered
    let distance_to_push_center =
        (max_viewport_size - min_viewport_size) / 2.0;
    let center_push =
        distance_to_push_center / min_viewport_size * 2.0;

    let mut coord =
        (uv * width_height / min_viewport_size * 2.0) - 1.0;
    if viewport_width > viewport_height {
        coord.x -= center_push;
    } else if viewport_width < viewport_height {
        coord.y -= center_push;
    };

    coord
}
