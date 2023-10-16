use bevy::prelude::*;
use std::f32::consts::FRAC_PI_4;

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

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn system(mut gizmos: Gizmos, time: Res<Time>) {
    let ray_direction = Vec2::from_angle(FRAC_PI_4 - 0.3);

    let ray = Ray {
        origin: Vec2::splat(0.),
        direction: ray_direction,
    };

    gizmos.ray_2d(ray.origin, ray.at(800.), Color::GREEN);

    // only send out 10 rays max
    // if it takes more than 10 rays to hit something,
    // then we're out of luck

    let MAX_STEPS =
        (time.elapsed_seconds() % 10.).floor() as i32;
    let mut dist = 0.0;
    for i in 0..MAX_STEPS {
        let current_pos = ray.at(dist);

        let dist_to_sdf = scene(current_pos, &mut gizmos);

        gizmos.ray_2d(
            current_pos,
            ray_direction * dist_to_sdf,
            if i == MAX_STEPS - 1 {
                Color::BLUE
            } else {
                Color::NONE
            },
        );

        // if we're close to anything, consider the ray to
        // have "hit" it and stop iterating.
        if dist_to_sdf < 0.1 {
            break;
        }

        gizmos.circle_2d(current_pos, 1.0, Color::BLUE);

        gizmos.circle_2d(
            current_pos,
            dist_to_sdf,
            if i == MAX_STEPS - 1 {
                Color::BLUE
            } else {
                Color::NONE
            },
        );
        dist = dist + dist_to_sdf;

        // if we've passed the scene, stop iterating.
        // Don't want the ray going off forever
        // into the distance
        if dist > 600. {
            break;
        }
    }
    // hack to render gizmos for sdf shapes
    scene(Vec2::new(0., 0.), &mut gizmos);
}

fn scene(point: Vec2, gizmos: &mut Gizmos) -> f32 {
    let position = Vec2::new(30., 100.);
    let radius = 100.;
    let rect =
        sd_rect(point, position, Vec2::splat(radius));
    gizmos.rect_2d(
        position,
        0.,
        Vec2::splat(radius),
        Color::WHITE,
    );
    rect
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

fn sd_rect(point: Vec2, center: Vec2, size: Vec2) -> f32 {
    let recentered_point = point - center;
    let d = recentered_point.abs() - (size / 2.0);
    d.max(Vec2::splat(0.0)).length() + d.x.max(d.y).min(0.0)
}
