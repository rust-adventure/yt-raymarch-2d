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
        .insert_resource(CurrentShape::Circle)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_mouse,
                ray_gizmos,
                update_shape,
                button_system,
            ),
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
                text.sections[1].value =
                    format!("{:+}", coord.x);
                text.sections[3].value =
                    format!("{:+}", coord.y);
                text.sections[5].value =
                    format!("{:+}", coord.xy().length());
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

        gizmos.line_2d(
            Vec2::ZERO,
            Vec2::new(x, y),
            Color::GREEN,
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
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Px(20.)),
                        ..default()
                    },
                    text: Text::from_sections(vec![
                        TextSection {
                            value: "mouse_position (end of line)\n  x: "
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
                            value: "\n\ndistance_to_center:\n  ".to_string(),
                            style: text_style.clone(),
                        }, TextSection {
                            value: "".to_string(),
                            style: green_text.clone(),
                        },
                    ]),
                    ..default()
                },
                MousePosition,
            ));

            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Auto,
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    border_color: BorderColor(Color::rgb(0.2, 0.2, 0.2)),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Change Shape",
                        TextStyle {
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

/// The Material trait is very configurable, but
/// comes with sensible defaults for all methods.
/// You only need to implement functions for
/// features that need non-default behavior. See
/// the Material api docs for details!
impl Material2d for SdfDemoMaterial {
    fn fragment_shader() -> ShaderRef {
        "single-sdf-distance-as-gradient-with-mouse.wgsl"
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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_shape: ResMut<CurrentShape>,
) {
    for (
        interaction,
        mut color,
        mut border_color,
        _children,
    ) in &mut interaction_query
    {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::WHITE;
                *current_shape = match *current_shape {
                    CurrentShape::Circle => {
                        CurrentShape::Box
                    }
                    CurrentShape::Box => {
                        CurrentShape::EquilateralTriangle
                    }
                    CurrentShape::EquilateralTriangle => {
                        CurrentShape::X
                    }
                    CurrentShape::X => CurrentShape::CoolS,
                    CurrentShape::CoolS => {
                        CurrentShape::Circle
                    }
                };
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
