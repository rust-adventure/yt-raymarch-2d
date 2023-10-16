use crate::bevy_plugin_shader2d::Shader2dWindowPlugin;
use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

pub fn app() {
    App::new()
        .add_plugins((Shader2dWindowPlugin {
            shader: SdfDemoMaterial {
                color: Color::BLUE,
                mouse: Vec2::splat(0.),
            },
        },))
        .add_systems(Update, (update_mouse,))
        .run();
}

fn update_mouse(
    window: Query<&Window, Changed<Window>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut materials: ResMut<Assets<SdfDemoMaterial>>,
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
        }
    }
}

impl Material2d for SdfDemoMaterial {
    fn fragment_shader() -> ShaderRef {
        "single-sdf-distance-as-circle.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct SdfDemoMaterial {
    #[uniform(0)]
    color: Color,
    #[uniform(0)]
    mouse: Vec2,
}
