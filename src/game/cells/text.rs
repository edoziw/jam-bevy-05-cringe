use crate::game::{
    assets::{HandleMap, Scene3dKey},
    biology::{Codon, LogTextRes, Mrna, CODON_TABLE},
    spawn::codon_chart::SpawnCodonChart,
};
use bevy::prelude::*;
pub fn spawn_codon_gltfs(
    _trigger: Trigger<SpawnCodonChart>,
    mut commands: Commands,
    scene_handles: Res<HandleMap<Scene3dKey>>,
) {
    // note that we have to include the `Scene0` label
    let radius = 15.;
    let scale = 2.;
    // to position our 3d model, simply use the Transform
    // in the SceneBundle
    let angle_rad = -2. * std::f32::consts::PI / 64.;
    let mut translation = Vec3::new(0., radius, 0.);
    //half angle to line up to before the first base
    translation = rotate_by_angle_rad(&mut translation, angle_rad / 2. - angle_rad);

    for i in 0..64 {
        translation = rotate_by_angle_rad(&mut translation, angle_rad);

        let c = CODON_TABLE.translation(&Codon::from_index(i));
        if c == ' ' {
            continue;
        }
        let scene_3d_key = CODON_TABLE.scene_3d_key(c);
        info!("{} {:?}", c, scene_3d_key);
        commands.spawn((
            SceneBundle {
                scene: scene_handles[&scene_3d_key].clone_weak(),
                transform: Transform::from_translation(translation).with_scale(Vec3::splat(scale)),
                ..Default::default()
            },
            Name::new(c.to_string()),
        ));
    }
}

pub fn spawn_acid(commands: &Commands, mrna: &Mrna, log_text: &mut LogTextRes) {
    info!("spawn_acid {:?}", mrna);
    log_text.text = format!("{}\n{:?}", log_text.text, mrna);
}

fn rotate_by_angle_rad(vec: &mut Vec3, angle_rad: f32) -> Vec3 {
    let t = Transform::from_rotation(Quat::from_rotation_z(angle_rad));
    t.transform_point(*vec)
}

/*
/// Helper resource for tracking our asset
#[derive(Resource)]
struct MyAssetPack(Handle<Gltf>);

fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("my_asset_pack.glb");
    commands.insert_resource(MyAssetPack(gltf));
}

fn spawn_gltf_objects(
    mut commands: Commands,
    my: Res<MyAssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // spawn the first scene in the file
        commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..Default::default()
        });

        // spawn the scene named "YellowCar"
        commands.spawn(SceneBundle {
            scene: gltf.named_scenes["YellowCar"].clone(),
            transform: Transform::from_xyz(1.0, 2.0, 3.0),
            ..Default::default()
        });

        // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
}
 */
