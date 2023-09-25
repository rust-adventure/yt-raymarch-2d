//! This example demonstrates Bevy's immediate
//! mode drawing API intended for visual
//! debugging.

use std::f32::consts::PI;

use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(
            0.9, 0.3, 0.6,
        )))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (system,))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    let center_radius = 10.;
    // The circles have 32 line-segments by default.
    gizmos.circle_2d(
        Vec2::splat(0.),
        center_radius,
        Color::BLACK,
    );

    let angle = (time.elapsed_seconds() * 0.25).sin() * PI;
    let ray_direction = Vec2::from_angle(angle);

    let ray = Ray {
        origin: center_radius * ray_direction,
        direction: ray_direction,
    };

    gizmos.ray_2d(
        center_radius * ray_direction,
        ray_direction * 80.,
        Color::GREEN,
    );

    // only send out 10 rays max
    // if it takes more than 10 rays to hit something, then we're out
    // of luck
    let MAX_STEPS = 10;
    let mut dist = 0.0;
    for i in 0..MAX_STEPS {
        let current_pos = ray.at(dist);

        let dist_to_sdf = scene(current_pos, &mut gizmos);

        gizmos.ray_2d(
            current_pos,
            ray_direction * dist_to_sdf,
            Color::RgbaLinear {
                red: 0.0,
                green: 0.0,
                blue: i as f32 / (MAX_STEPS - 1) as f32,
                alpha: 1.0,
            },
        );

        // if we're close to anything, consider the ray to have
        // "hit" it and stop iterating.
        if dist_to_sdf < 0.1 {
            break;
        }

        gizmos.circle_2d(current_pos, 1.0, Color::BLUE);

        gizmos.circle_2d(
            current_pos,
            dist_to_sdf,
            Color::BLUE,
        );
        dist = dist + dist_to_sdf;

        // if we've passed the scene, stop iterating. Don't want the ray
        // going off forever into the distance
        if dist > 350. {
            break;
        }
    }
}

fn scene(point: Vec2, gizmos: &mut Gizmos) -> f32 {
    // circle 1
    let position = 40.;
    let radius = 10.;
    let circle_one =
        sd_circle(point, Vec2::splat(position), radius);
    gizmos.circle_2d(
        Vec2::splat(position),
        radius,
        Color::WHITE,
    );

    // circle 2
    let position = Vec2::new(200., 50.);
    let radius = 20.;
    let circle_two = sd_circle(point, position, radius);
    gizmos.circle_2d(position, radius, Color::WHITE);

    // circle 3
    let position = Vec2::new(-50., 300.);
    let radius = 10.;
    let circle_three = sd_circle(point, position, radius);
    gizmos.circle_2d(position, radius, Color::WHITE);

    // .min for each circle means we get the closest circle distance
    circle_one.min(circle_two).min(circle_three)
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

fn sd_circle(
    point: Vec2,
    center: Vec2,
    radius: f32,
) -> f32 {
    // point - center is so that we can "relocate" a circle
    // because otherwise it would only exist at world origin: 0,0
    (point - center).length() - radius
}
