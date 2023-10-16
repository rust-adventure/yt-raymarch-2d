use crate::{
    bevy_plugin_shader2d::Shader2dWindowPlugin, *,
};
use bevy::{
    math::Vec2Swizzles,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Resource)]
enum CurrentShape {
    Circle,
    Box,
    EquilateralTriangle,
    X,
    CoolS,
}

impl CurrentShape {
    fn as_u32(&self) -> u32 {
        match self {
            CurrentShape::Circle => 1,
            CurrentShape::Box => 2,
            CurrentShape::EquilateralTriangle => 3,
            CurrentShape::X => 4,
            CurrentShape::CoolS => 5,
        }
    }
}

pub fn app() {
    App::new()
        .add_plugins((Shader2dWindowPlugin {
            shader: SdfDemoMaterial {
                color: Color::BLUE,
                mouse: Vec2::splat(0.),
                shape: 1,
            },
        },))
        .insert_resource(CurrentShape::Box)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_mouse, ray_gizmos, update_shape),
        )
        .run();
}

fn update_shape(
    mut materials: ResMut<Assets<SdfDemoMaterial>>,
    current_shape: Res<CurrentShape>,
) {
    if current_shape.is_changed() {
        for (_handle, mat) in materials.iter_mut() {
            mat.shape = current_shape.as_u32()
        }
    }
}

fn update_mouse(
    window: Query<&Window, Changed<Window>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut materials: ResMut<Assets<SdfDemoMaterial>>,
    mut text: Query<&mut Text, With<MousePosition>>,
) {
    if window.is_empty() {
        return;
    };
    let resolution = &window.single().resolution;
    for event in cursor_moved_events.iter() {
        for (_handle, mat) in materials.iter_mut() {
            mat.mouse = Vec2::new(
                event.position.x / resolution.width(),
                event.position.y / resolution.height(),
            );
            for mut text in text.iter_mut() {
                let coord = center_sdf(
                    mat.mouse.xy(),
                    Vec2::new(
                        resolution.width(),
                        resolution.height(),
                    ),
                );
                // let distance_from_circle =
                // sd_circle(coord);
                text.sections[1].value =
                    format!("{:+}", coord.x);
                text.sections[3].value =
                    format!("{:+}", coord.y);
                text.sections[5].value =
                    format!("{:+}", coord.x.abs());
                text.sections[7].value =
                    format!("{:+}", coord.y.abs());

                let point = coord;
                let size = Vec2::new(0.5, 0.5);
                let abs = point.abs();
                let distance_vector = abs - size;

                let length = distance_vector
                    .max(Vec2::ZERO)
                    .length();

                let offset = distance_vector
                    .x
                    .max(distance_vector.y)
                    .min(0.0);

                let final_distance = length + offset;

                text.sections[9].value = format!(
                    "x: {:+}\n  y: {:+}",
                    distance_vector.x, distance_vector.y
                );
                text.sections[11].value =
                    format!("{:+}", length);
                text.sections[13].value =
                    format!("{:+}", offset);
                text.sections[15].value =
                    format!("{:+}", final_distance);
                // text.sections[5].value =
                //     format!("{:+}",
                // distance_from_circle);
            }
        }
    }
}
fn ray_gizmos(
    window: Query<&Window>,
    materials: ResMut<Assets<SdfDemoMaterial>>,
    mut gizmos: Gizmos,
) {
    if window.is_empty() {
        return;
    };
    let resolution = &window.single().resolution;
    for (_handle, mat) in materials.iter() {
        let uv = mat.mouse.xy() * 2.0 - 1.0;

        let x = uv.x * resolution.width() / 2.0;
        let y = -uv.y * resolution.height() / 2.0;

        let coord = center_sdf(
            mat.mouse.xy(),
            Vec2::new(
                resolution.width(),
                resolution.height(),
            ),
        );

        let width_height = Vec2::new(
            resolution.width(),
            resolution.height(),
        );

        // let distance_from_circle = sd_circle(coord);

        // let real_distance = distance_from_circle
        //     * width_height.x.min(width_height.y)
        //     / 2.0;
        // // let direction = Vec2::new(x, y).normalize();
        // // dbg!(distance_from_circle, direction);

        // let direction = Vec2::new(x, y).normalize();

        let point = coord;
        let size = Vec2::new(0.5, 0.5);
        let abs = point.abs();
        let distance_vector = abs - size;
        gizmos.line_2d(
            Vec2::new(0., 0.)
                + size * width_height.x.min(width_height.y)
                    / 2.0,
            (distance_vector.max(Vec2::ZERO) + size)
                * width_height.x.min(width_height.y)
                / 2.0,
            Color::Rgba {
                red: 0.122,
                green: 0.663,
                blue: 0.957,
                alpha: 1.,
            },
        );
        let length =
            distance_vector.max(Vec2::ZERO).length();

        let offset = distance_vector
            .x
            .max(distance_vector.y)
            .min(0.0);

        if offset < 0. {
            let render_offset = offset
                * width_height.x.min(width_height.y)
                / 2.0;
            gizmos.ray_2d(
                Vec2::new(x, y).abs(),
                if distance_vector.x <= distance_vector.y {
                    Vec2::new(0., -render_offset)
                } else {
                    Vec2::new(-render_offset, 0.)
                },
                Color::YELLOW,
            );
        }

        let final_distance = length + offset;

        gizmos.circle_2d(
            Vec2::new(x, y).abs(),
            5.0,
            Color::GREEN,
        );

        gizmos.circle_2d(
            Vec2::new(x, y),
            5.0,
            Color::WHITE,
        );
        gizmos.line_2d(
            Vec2::new(-resolution.width(), 0.),
            Vec2::new(resolution.width(), 0.),
            Color::WHITE,
        );
        gizmos.line_2d(
            Vec2::new(0., -resolution.height()),
            Vec2::new(0., resolution.height()),
            Color::WHITE,
        );
    }
}

#[derive(Component)]
struct MousePosition;

fn setup(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: 20.0,
        color: Color::WHITE,
        ..default()
    };
    let green_text = TextStyle {
        font_size: 20.0,
        color: Color::GREEN,
        ..default()
    };
    let yellow_text = TextStyle {
        font_size: 20.0,
        color: Color::YELLOW,
        ..default()
    };
    let blue_text = TextStyle {
        font_size: 20.0,
        color: Color::Rgba {
            red: 0.122,
            green: 0.663,
            blue: 0.957,
            alpha: 1.,
        },
        ..default()
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(20.),
                top: Val::Px(20.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|parent| { parent.spawn((
            TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(20.)),
                    ..default()
                },
                text: Text::from_sections(vec![
                    TextSection {
                        value: "mouse_position\n  x: "
                            .to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n  y: ".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_style.clone(),
                    },
                    TextSection {
                        value: "\n\nmouse_position (moved to first quadrant)\n  x: "
                        .to_string(),
                    style: green_text.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: green_text.clone(),
                },
                TextSection {
                    value: "\n  y: ".to_string(),
                    style: green_text.clone(),
                },
                TextSection {
                    value: "".to_string(),
                    style: green_text.clone(),
                },

                TextSection {
                    value: "\n\npoint.abs() - size (distance_vector):\n  "
                    .to_string(),
                style: blue_text.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: blue_text.clone(),
            },
            TextSection {
                value: "\ndistance.max(Vec2::ZERO).length():\n  ".to_string(),
                style: blue_text.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: blue_text.clone(),
            },
            TextSection {
                value: "\n\ndistance.x.max(distance.y).min(0.0):\n  ".to_string(),
                style: yellow_text.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: yellow_text.clone(),
            },
            TextSection {
                value: "\n\nfinal_distance:\n  ".to_string(),
                style: text_style.clone(),
            },
            TextSection {
                value: "".to_string(),
                style: text_style.clone(),
            },
                ]),
                ..default()
            },
            MousePosition,
        ));
    });
}

/// The Material trait is very configurable, but
/// comes with sensible defaults for all methods.
/// You only need to implement functions for
/// features that need non-default behavior. See
/// the Material api docs for details!
impl Material2d for SdfDemoMaterial {
    fn fragment_shader() -> ShaderRef {
        "single-sdf-distance-as-gradient-with-algorithm.wgsl"
            .into()
    }
}

// This is the struct that will be passed to your
// shader
#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct SdfDemoMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    shape: u32,
    #[uniform(0)]
    mouse: Vec2,
}

fn sd_circle(point: Vec2) -> f32 {
    let center = Vec2::splat(0.);
    let radius = 0.5;
    // point - center is so that we can "relocate" a
    // circle because otherwise it would only
    // exist at world origin: 0,0
    point.distance(center) - radius
}

fn sd_box(point: Vec2) -> f32 {
    let size = Vec2::new(0.5, 0.5);
    let distance = point.abs() - size;
    distance.max(Vec2::ZERO).length()
        + distance.x.max(distance.y).min(0.0)
}
