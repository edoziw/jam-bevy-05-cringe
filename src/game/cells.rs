use bevy::prelude::*;

mod cell;
mod text;
use cell::{draw_cell, CellBundle, EyeBundle};
use text::spawn_gltf;

pub(super) fn plugin(app: &mut App) {
    //app.init_resource::<MyAssetPack>();
    app.add_systems(Startup, (spawn_gltf, spawn_cells).chain());
    app.add_systems(Update, draw_cell);
}

fn spawn_cells(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let x = 8.;
    commands
        .spawn(CellBundle::new(Transform::from_xyz(x, 0., 0.)))
        .with_children(|parent| {
            parent.spawn(EyeBundle::new(&mut meshes, &mut materials));
        });
    commands
        .spawn(CellBundle::new(Transform::from_xyz(-x, 0., 0.)))
        .with_children(|parent| {
            parent.spawn(EyeBundle::new(&mut meshes, &mut materials));
        });
}
