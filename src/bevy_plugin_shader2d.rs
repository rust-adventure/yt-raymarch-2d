use bevy::{
    asset::ChangeWatcher,
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
        Mesh2dHandle,
    },
    window::WindowResolution,
};
use core::hash::Hash;
use std::{f32::INFINITY, time::Duration};

pub struct Shader2dWindowPlugin<S: Material2d> {
    pub shader: S,
}

impl<M: Material2d> Plugin for Shader2dWindowPlugin<M>
where
    <M as AsBindGroup>::Data: PartialEq<<M as AsBindGroup>::Data>
        + Eq
        + Hash
        + Clone,
{
    fn build(&self, app: &mut App) {
        let mat = Material2dPlugin::<M>::default();
        app.insert_resource(UserShader(
            self.shader.clone(),
        ))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: Some(
                        ChangeWatcher {
                            delay: Duration::from_millis(
                                200,
                            ),
                        },
                    ),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // title: todo!(),
                        resolution: WindowResolution::new(
                            300., 168.75,
                        ),
                        resize_constraints:
                            WindowResizeConstraints {
                                min_width: 300.,
                                min_height: 300.,
                                max_width: INFINITY,
                                max_height: INFINITY,
                            },
                        resizable: true,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins((mat,))
        .add_systems(Startup, setup::<M>)
        .add_systems(Update, (update_window,));
    }
}

#[derive(Component)]
struct WindowCover;

fn update_window(
    window: Query<&Window, Changed<Window>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut entities: Query<
        &mut Mesh2dHandle,
        With<WindowCover>,
    >,
) {
    if window.is_empty() {
        return;
    };
    let resolution = &window.single().resolution;
    let new_mesh = Mesh2dHandle::from(meshes.add(
        Mesh::from(shape::Quad {
            size: Vec2::new(
                resolution.width(),
                resolution.height(),
            ),
            ..default()
        }),
    ));
    for mut handle in entities.iter_mut() {
        *handle = new_mesh.clone();
    }
}

#[derive(Resource)]
pub struct UserShader<S: Material2d>(S);

fn setup<S: Material2d>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<S>>,
    shader: Res<UserShader<S>>,
) {
    // cube
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle::from(meshes.add(
                Mesh::from(shape::Quad {
                    size: Vec2::splat(4000.0),
                    ..default()
                }),
            )),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(shader.0.clone()),
            ..default()
        },
        WindowCover,
    ));

    // camera
    commands.spawn(Camera2dBundle::default());
}
