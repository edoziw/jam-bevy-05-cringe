use bevy::{
    color::palettes::css::{ORANGE, WHITE},
    math::vec3,
    prelude::*,
    render::view::visibility::RenderLayers,
};
use noise::{NoiseFn, OpenSimplex};

#[derive(Component)]
pub struct Curve {
    curve: CubicBezier<Vec3>,
    noise: OpenSimplex,
}

#[derive(Component)]
pub struct Eye {}

impl Default for Curve {
    fn default() -> Self {
        let noise = OpenSimplex::default();
        Self {
            curve: CubicBezier::new(points_circle()),
            noise,
        }
    }
}
impl Curve {
    fn get_noised_curve(
        &self,
        time: &Res<Time>,
        global_transform: &GlobalTransform,
    ) -> CubicBezier<Vec3> {
        let no_noise_points = points_circle();
        let mut bezier = self.curve.clone();
        bezier
            .control_points
            .iter_mut()
            .enumerate()
            .for_each(|(index_points, points)| {
                points
                    .iter_mut()
                    .enumerate()
                    .for_each(|(index_point, point)| {
                        //transform the point without noise
                        let transformed_point = global_transform
                            .transform_point(no_noise_points[index_points][index_point]);
                        //add noise to the point
                        *point = noise_point(&self.noise, time, transformed_point);
                    });
            });
        bezier
    }
}
// https://spencermortensen.com/articles/bezier-circle/
fn points_circle() -> Vec<[Vec3; 4]> {
    const Z: f32 = 0.0;
    const A: f32 = 1.00005519;
    const B: f32 = 0.55342686;
    const C: f32 = 0.99873585;
    const SHARED: [Vec3; 4] = [vec3(Z, A, Z), vec3(A, Z, Z), vec3(Z, -A, Z), vec3(-A, Z, Z)];
    let mut result = Vec::new();
    result.push([SHARED[0], vec3(B, C, Z), vec3(C, B, Z), SHARED[1]]);
    result.push([SHARED[1], vec3(C, -B, Z), vec3(B, -C, Z), SHARED[2]]);
    result.push([SHARED[2], vec3(-B, -C, Z), vec3(-C, -B, Z), SHARED[3]]);
    result.push([SHARED[3], vec3(-C, B, Z), vec3(-B, C, Z), SHARED[0]]);
    result
}

fn noise_point(noise_fn: &OpenSimplex, time: &Res<Time>, point: Vec3) -> Vec3 {
    let t = time.elapsed_seconds() as f64;
    let noise_x = noise_fn.get([point.x as f64, point.y as f64, point.z as f64, t]);
    let noise_y = noise_fn.get([point.x as f64, point.y as f64, point.z as f64, t + 7.0]);
    let noise_z = noise_fn.get([point.x as f64, point.y as f64, point.z as f64, t + 13.0]);
    vec3(
        point.x + noise_x as f32,
        point.y + noise_y as f32,
        point.z + noise_z as f32,
    )
}
#[derive(Bundle)]
pub struct EyeBundle {
    pbr_bundle: PbrBundle,
    name: Name,
    eye: Eye,
}
impl EyeBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> Self {
        Self {
            pbr_bundle: PbrBundle {
                mesh: meshes.add(Cuboid::default()),
                material: materials.add(Color::from(ORANGE)),
                ..default()
            },
            name: Name::new("eye"),
            eye: Eye {},
        }
    }
}
#[derive(Bundle)]
pub struct CellBundle {
    curve: Curve,
    name: Name,
    spatial_bundle: SpatialBundle,
    render_layer: RenderLayers,
}

impl CellBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            spatial_bundle: SpatialBundle {
                transform,
                ..default()
            },
            curve: Curve::default(),
            name: Name::new("cell"),
            render_layer: RenderLayers::layer(1),
        }
    }
}

pub fn draw_cell(
    time: Res<Time>,
    mut eye_q: Query<(&Parent, &mut Transform), With<Eye>>,
    mut cell_q: Query<(&GlobalTransform, &Curve)>,
    mut gizmos: Gizmos,
) {
    let t = (time.elapsed_seconds().sin() + 1.0) / 2.0;
    for (parent, mut transform) in &mut eye_q {
        match cell_q.get_mut(parent.get()) {
            Ok((global_transform, curve)) => {
                let mut curve = curve.get_noised_curve(&time, &global_transform);
                // draw the curve
                gizmos.linestrip(curve.to_curve().iter_positions(50), WHITE);
                // un transform the curve to the local space of the eye

                transform_curve(
                    &mut curve,
                    &Transform::from_matrix(
                        global_transform
                            .reparented_to(&GlobalTransform::IDENTITY)
                            .compute_matrix()
                            .inverse(),
                    ),
                );
                let curve_cubic = curve.to_curve();
                transform.translation =
                    curve_cubic.position(t * curve_cubic.segments().len() as f32);
            }
            Err(_) => {}
        }
    }
}

fn transform_curve(curve: &mut CubicBezier<Vec3>, transform: &Transform) {
    curve.control_points.iter_mut().for_each(|control_points| {
        control_points
            .iter_mut()
            .for_each(|point| *point = transform.transform_point(*point));
    });
}
