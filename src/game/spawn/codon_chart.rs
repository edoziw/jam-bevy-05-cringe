//! Spawn the codon chart.

use bevy::{math::vec3, prelude::*, render::view::visibility::RenderLayers};
use bevy_sprite3d::{Sprite3d, Sprite3dParams};

use crate::{
    game::assets::{HandleMap, ImageKey},
    game::cells::text::spawn_codon_gltfs,
    screen::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.observe(spawn_codon_chart);
    app.observe(spawn_codon_gltfs);
    app.register_type::<CodonChart>();
}

#[derive(Event, Debug)]
pub struct SpawnCodonChart;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct CodonChart;

fn spawn_codon_chart(
    _trigger: Trigger<SpawnCodonChart>,
    mut commands: Commands,
    image_handles: Res<HandleMap<ImageKey>>,
    mut sprite_params: Sprite3dParams,
) {
    let fudge = 40.;
    commands.spawn((
        Name::new("CodonChart"),
        CodonChart,
        Sprite3d {
            image: image_handles[&ImageKey::CodonChart].clone_weak(),
            pixels_per_metre: 400.,
            transform: Transform::from_translation(vec3(0.0, 0.7, -fudge))
                .with_scale(vec3(fudge, fudge, 1.0)),
            ..default()
        }
        .bundle(&mut sprite_params),
        RenderLayers::layer(0),
        StateScoped(Screen::Playing),
    ));
}
