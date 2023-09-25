//! This example demonstrates Bevy's immediate mode drawing API intended for visual debugging.

use std::f32::consts::{FRAC_2_PI, PI, TAU};

use bevy::{
    math::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles},
    prelude::{
        shape::{Circle, Quad, RegularPolygon},
        *,
    },
    sprite::MaterialMesh2dBundle,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.9, 0.3, 0.6,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system, move_viewer))
        .run();
}

fn move_viewer(
    mut viewer: Query<&mut Transform, With<Viewer>>,
    time: Res<Time>,
) {
    let mut viewer = viewer.single_mut();
    let x = time.elapsed_seconds().cos();
    let y = (2. * time.elapsed_seconds()).sin() / 2.;
    viewer.translation.x = x * 200.;
    viewer.translation.y = y * 200.;
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpatialBundle::default(), Viewer));

    let circle = shape::Circle::new(10.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(10.).into())
                .into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(40., 40., 0.),
            ),
            ..default()
        },
        Sdf::from(circle),
    ));

    let circle = shape::Circle::new(20.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Circle::new(20.).into())
                .into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(200., 50., 0.),
            ),
            ..default()
        },
        Sdf::from(circle),
    ));

    let circle = shape::Circle::new(10.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(circle.into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(-50., 300., 0.),
            ),
            ..default()
        },
        Sdf::from(circle),
    ));

    let quad = shape::Quad::new(Vec2::new(50.0, 100.0));
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(quad.into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(-200., 200., 0.),
            ),
            ..default()
        },
        Sdf::from(quad),
    ));

    let pentagon = shape::RegularPolygon {
        radius: 30.,
        sides: 5,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(pentagon.into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(-200., -200., 0.),
            ),
            ..default()
        },
        Sdf::from(pentagon),
    ));

    let hexagon = shape::RegularPolygon {
        radius: 40.,
        sides: 6,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(hexagon.into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(-100., -200., 0.),
            ),
            ..default()
        },
        Sdf::from(hexagon),
    ));

    let octogon = shape::RegularPolygon {
        radius: 40.,
        sides: 8,
    };
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(octogon.into()).into(),
            material: materials.add(ColorMaterial::from(
                Color::Hsla {
                    hue: 90.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.0,
                },
            )),
            transform: Transform::from_translation(
                Vec3::new(0., -200., 0.),
            ),
            ..default()
        },
        Sdf::from(octogon),
    ));
}

#[derive(Component)]
enum Sdf {
    Circle(Circle),
    Quad(Quad),
    RegularPolygon(RegularPolygon),
}

impl From<Circle> for Sdf {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}
impl From<Quad> for Sdf {
    fn from(value: Quad) -> Self {
        Self::Quad(value)
    }
}

impl From<RegularPolygon> for Sdf {
    fn from(value: RegularPolygon) -> Self {
        if ![4, 5, 6, 8].contains(&value.sides) {
            panic!(
                "Unsupported sdf with {} sides",
                value.sides
            );
        } else {
            Self::RegularPolygon(value)
        }
    }
}

impl Sdf {
    fn dist(&self, point: &Vec2, center: &Vec2) -> f32 {
        let recentered_point = *point - *center;
        match self {
            Sdf::Circle(Circle { radius, .. }) => {
                recentered_point.length() - radius
            }
            Sdf::Quad(Quad { size, .. }) => {
                let d =
                    recentered_point.abs() - (*size / 2.0);
                d.max(Vec2::splat(0.0)).length()
                    + d.x.max(d.y).min(0.0)
            }
            // Pentagon
            Sdf::RegularPolygon(RegularPolygon {
                radius,
                sides,
            }) if *sides == 5 => {
                let k = Vec3::new(
                    0.809016994,
                    0.587785252,
                    0.726542528,
                );
                let point = Vec2::new(
                    recentered_point.x.abs(),
                    recentered_point.y,
                );
                let point = point
                    - 2.0
                        * Vec2::new(-k.x, k.y)
                            .dot(point)
                            .min(0.0)
                        * Vec2::new(-k.x, k.y);
                let point = point
                    - 2.0
                        * Vec2::new(k.x, k.y)
                            .dot(point)
                            .min(0.0)
                        * Vec2::new(k.x, k.y);
                let point = point
                    - Vec2::new(
                        point.x.clamp(
                            -radius * k.z,
                            radius * k.z,
                        ),
                        *radius,
                    );

                (point).length() * point.y.signum()
            }
            // Hexagon
            Sdf::RegularPolygon(RegularPolygon {
                radius,
                sides,
            }) if *sides == 6 => {
                let k = Vec3::new(
                    -0.866025404,
                    0.5,
                    0.577350269,
                );
                let point = recentered_point.abs();
                let point = point
                    - 2.0
                        * k.xy().dot(point).min(0.0)
                        * k.xy();
                let point = point
                    - Vec2::new(
                        point.x.clamp(
                            -k.z * radius,
                            k.z * radius,
                        ),
                        *radius,
                    );
                point.length() * point.y.signum()
            }
            Sdf::RegularPolygon(RegularPolygon {
                radius,
                sides,
            }) if *sides == 8 => {
                let k = Vec3::new(
                    -0.9238795325,
                    0.3826834323,
                    0.4142135623,
                );
                let point = recentered_point.abs();
                let point = point
                    - 2.0
                        * k.xy().dot(point).min(0.0)
                        * k.xy();
                let point = point
                    - 2.0
                        * Vec2::new(-k.x, k.y)
                            .dot(point)
                            .min(0.0)
                        * Vec2::new(-k.x, k.y);
                let point = point
                    - Vec2::new(
                        point.x.clamp(
                            -k.z * radius,
                            k.z * radius,
                        ),
                        *radius,
                    );
                point.length() * point.y.signum()
            }
            _ => panic!("unsupported Sdf"),
        }
    }
}

#[derive(Component)]
struct Viewer;

fn system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    scene: Query<(&Sdf, &Transform)>,
    viewer: Query<&Transform, With<Viewer>>,
) {
    if scene.is_empty() {
        return;
    }
    let viewer_location = viewer.single().translation.xy();
    let center_radius = 10.;
    // The circles have 32 line-segments by default.
    gizmos.circle_2d(
        viewer_location,
        center_radius,
        Color::WHITE,
    );

    let num_rays = 8;
    // let angle = (time.elapsed_seconds() * 0.25).sin() * PI;
    let angle_increment = TAU / num_rays as f32;
    for angle_i in 0..num_rays {
        let ray_direction = Vec2::from_angle(
            angle_i as f32 * angle_increment,
        );

        let ray = Ray {
            origin: viewer_location
                + center_radius * ray_direction,
            direction: ray_direction,
        };

        // debug ray position
        // gizmos.ray_2d(
        //     center_radius * ray_direction,
        //     ray_direction * 80.,
        //     Color::GREEN,
        // );

        let MAX_STEPS = 10;
        let mut dist = 0.0;
        for i in 0..MAX_STEPS {
            let current_pos = ray.at(dist);

            // query
            // let dist_to_sdf = scene(current_pos, &mut gizmos);
            let dist_to_sdf = scene
                .iter()
                .map(|(sdf, transform)| {
                    sdf.dist(
                        &current_pos,
                        &transform.translation.xy(),
                    )
                })
                .min_by(|a, b| {
                    a.partial_cmp(b)
                        .expect("expected no NaNs")
                })
                .expect(
                    "sdf array must have at least one sdf",
                );

            gizmos.ray_gradient_2d(
                current_pos,
                ray_direction * dist_to_sdf,
                Color::WHITE,
                Color::Hsla {
                    hue: 210.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 0.6,
                },
            );

            if dist_to_sdf < 0.001 {
                break;
            }

            gizmos.circle_2d(
                current_pos,
                1.0,
                Color::Hsla {
                    hue: 210.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.,
                },
            );

            gizmos.circle_2d(
                current_pos,
                dist_to_sdf,
                Color::Hsla {
                    hue: 210.2,
                    saturation: 0.754,
                    lightness: 0.602,
                    alpha: 1.,
                },
            );
            dist = dist + dist_to_sdf;

            // if we've passed the scene, stop
            if dist > 350. {
                break;
            }
        }
    }
}

struct Ray {
    origin: Vec2,
    direction: Vec2,
}

impl Ray {
    fn at(&self, time: f32) -> Vec2 {
        self.origin + self.direction * time
    }
}
